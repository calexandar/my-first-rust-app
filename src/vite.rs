use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;

use serde::Deserialize;

pub const ENTRY: &str = "resources/js/app.tsx";

#[derive(Clone, Debug)]
pub struct Vite {
    hot_file: PathBuf,
    manifest: PathBuf,
    build_url: String,
}

#[derive(Deserialize)]
struct Chunk {
    file: String,
    #[serde(default)]
    css: Vec<String>,
    #[serde(default)]
    imports: Vec<String>,
}

impl Default for Vite {
    fn default() -> Self {
        Self::new()
    }
}

impl Vite {
    pub fn new() -> Self {
        Self {
            hot_file: PathBuf::from("public/hot"),
            manifest: PathBuf::from("public/build/.vite/manifest.json"),
            build_url: "/build".to_owned(),
        }
    }

    pub fn hot(&self) -> Option<String> {
        let value = fs::read_to_string(&self.hot_file).ok()?;
        let url = value.trim().trim_end_matches('/');

        if is_local_url(url) {
            Some(url.to_owned())
        } else {
            tracing::warn!(url, "ignoring a hot file that is not a local dev server");
            None
        }
    }

    pub fn version(&self) -> String {
        if self.hot().is_some() {
            return "dev".to_owned();
        }

        match fs::read(&self.manifest) {
            Ok(bytes) => {
                let mut hasher = DefaultHasher::new();
                bytes.hash(&mut hasher);
                format!("{:x}", hasher.finish())
            }
            Err(_) => String::new(),
        }
    }

    pub fn tags(&self) -> String {
        if let Some(url) = self.hot() {
            return format!(
                r#"<script type="module">
import RefreshRuntime from "{url}/@react-refresh";
RefreshRuntime.injectIntoGlobalHook(window);
window.$RefreshReg$ = () => {{}};
window.$RefreshSig$ = () => (type) => type;
window.__vite_plugin_react_preamble_installed__ = true;
</script>
<script type="module" src="{url}/@vite/client"></script>
<script type="module" src="{url}/{ENTRY}"></script>"#
            );
        }

        let Some(manifest) = fs::read_to_string(&self.manifest)
            .ok()
            .and_then(|json| serde_json::from_str::<HashMap<String, Chunk>>(&json).ok())
        else {
            tracing::warn!(
                "no Vite manifest found: run `npm run build`, or `npm run dev` for the dev server"
            );
            return String::new();
        };

        tags_for(&manifest, ENTRY, &self.build_url)
    }
}

fn tags_for(manifest: &HashMap<String, Chunk>, entry: &str, build_url: &str) -> String {
    let Some(chunk) = manifest.get(entry) else {
        return String::new();
    };

    let mut seen = HashSet::from([entry]);
    let mut styles = Vec::new();
    let mut preloads = Vec::new();
    collect(manifest, chunk, &mut seen, &mut styles, &mut preloads);

    let mut tags = Vec::new();

    for css in &styles {
        tags.push(format!(
            r#"<link rel="stylesheet" href="{}">"#,
            asset(build_url, css)
        ));
    }

    for file in &preloads {
        tags.push(format!(
            r#"<link rel="modulepreload" href="{}">"#,
            asset(build_url, file)
        ));
    }

    tags.push(format!(
        r#"<script type="module" src="{}"></script>"#,
        asset(build_url, &chunk.file)
    ));

    tags.join("\n")
}

fn collect<'a>(
    manifest: &'a HashMap<String, Chunk>,
    chunk: &'a Chunk,
    seen: &mut HashSet<&'a str>,
    styles: &mut Vec<&'a str>,
    preloads: &mut Vec<&'a str>,
) {
    for import in &chunk.imports {
        if !seen.insert(import) {
            continue;
        }

        if let Some(imported) = manifest.get(import) {
            collect(manifest, imported, seen, styles, preloads);
            preloads.push(&imported.file);
        }
    }

    for css in &chunk.css {
        if !styles.contains(&css.as_str()) {
            styles.push(css);
        }
    }
}

fn asset(build_url: &str, file: &str) -> String {
    escape(&format!("{build_url}/{file}"))
}

fn is_local_url(url: &str) -> bool {
    if !url
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ":/.[]-_".contains(c))
    {
        return false;
    }

    let Some(rest) = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
    else {
        return false;
    };

    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host = match authority.rsplit_once(':') {
        Some((host, port)) if !port.is_empty() && port.bytes().all(|b| b.is_ascii_digit()) => host,
        _ => authority,
    };

    matches!(host, "localhost" | "127.0.0.1" | "[::1]")
}

fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for c in value.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            c => escaped.push(c),
        }
    }

    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(file: &str, css: &[&str], imports: &[&str]) -> Chunk {
        Chunk {
            file: file.into(),
            css: css.iter().map(|value| (*value).to_owned()).collect(),
            imports: imports.iter().map(|value| (*value).to_owned()).collect(),
        }
    }

    #[test]
    fn tags_include_imported_styles_and_preloads() {
        let manifest = HashMap::from([
            (
                ENTRY.to_owned(),
                chunk("app.js", &["app.css"], &["_shared.js"]),
            ),
            ("_shared.js".to_owned(), chunk("shared.js", &[], &[])),
        ]);

        let tags = tags_for(&manifest, ENTRY, "/build");

        assert!(tags.contains(r#"<link rel="stylesheet" href="/build/app.css">"#));
        assert!(tags.contains(r#"<link rel="modulepreload" href="/build/shared.js">"#));
        assert!(tags.contains(r#"<script type="module" src="/build/app.js"></script>"#));
    }

    #[test]
    fn only_local_dev_server_urls_are_trusted() {
        for url in [
            "http://localhost:5173",
            "http://127.0.0.1:5173",
            "https://[::1]:5173",
            "http://localhost",
        ] {
            assert!(is_local_url(url), "{url}");
        }

        for url in [
            "https://evil.example",
            "http://localhost.evil.example:5173",
            "http://localhost@evil.example",
            "javascript:alert(1)",
            "//localhost:5173",
            "http://localhost:5173/\"><script>",
        ] {
            assert!(!is_local_url(url), "{url}");
        }
    }
}
