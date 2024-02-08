#![allow(non_snake_case)]
use std::sync::Arc;

use axum::{extract::State, response::Html, routing::get, Router};
use tokio::net::TcpListener;

use dioxus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "0.0.0.0:3000";
    let state = Arc::new(HtmlConfig::new("lostdisplay.space"));
    let listener = TcpListener::bind(addr).await?;
    let app = Router::new().route("/", get(root)).with_state(state);

    axum::serve(listener, app).await
}

struct HtmlConfig {
    title: String,
}

impl HtmlConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

fn pack_nodes(cfg: &HtmlConfig, nodes: LazyNodes) -> String {
    let body = dioxus_ssr::render_lazy(nodes);
    format!(
        include_str!("index.html"), title = cfg.title, body = body,
        head = "",
    )
}

async fn root(state: State<Arc<HtmlConfig>>) -> Html<String> {
    Html(pack_nodes(&state, rsx! {App{}}))
}

#[derive(Debug, Props, PartialEq)]
struct LinkProps<'a> {
    name: &'a str,
}

fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    render! {
        div {
            class: "text-red",
            "{cx.props.name}"
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        Link { name: "test" }
    }
}
