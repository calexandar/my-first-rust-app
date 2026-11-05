use inertia::{Inertia, props};

pub async fn index(inertia: Inertia) -> inertia::Response {
    inertia.render(
        "About",
        props! {
            message => "This page is routed by Axum and rendered by React through Inertia.",
        },
    )
}
