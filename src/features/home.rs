use inertia::{Inertia, props};

pub async fn index(inertia: Inertia) -> inertia::Response {
    inertia.render(
        "Home",
        props! {
            message => "Hello from Rust, React, and Inertia.",
        },
    )
}
