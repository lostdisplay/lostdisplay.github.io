#![allow(non_snake_case)]

use dioxus::prelude::*;

const PIXEL_STYLE: &'static str = "-ms-interpolation-mode: nearest-neighbor; image-rendering: -webkit-optimize-contrast; image-rendering: -moz-crisp-edges; image-rendering: -o-pixelated; image-rendering: pixelated;";
const LINKS: [LinkProps<'static>; 4] = [
    LinkProps::new("itch.png", "https://lostdisplay.itch.io"),
    LinkProps::new("youtube.png", "https://www.youtube.com/channel/UCdCRgJar9KbYnFNc4Vh_TIA"),
    LinkProps::new("kofi.png", "https://ko-fi.com/lostdisplay"),
    LinkProps::new("mastodon.png", "https://mastodon.gamedev.place/@lostdisplay"),
];

#[derive(Debug, Props, PartialEq)]
struct LinkProps<'a> {
    url: &'a str,
    icon: &'a str,
}

impl<'a> LinkProps<'a> {
    pub const fn new(icon: &'a str, url: &'a str) -> Self {
        Self { icon, url }
    }
}


// Main entry point
fn main() {
    dioxus_web::launch(App);
}

fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    render! {
        a {
            class: "hover:-translate-y-1",
            href: "{cx.props.url}",
            rel: "me",
            img {
                class: "h-16",
                style: "{PIXEL_STYLE}",
                src: "/{cx.props.icon}"
            }
        }
    }
}

fn Header(cx: Scope) -> Element {
    render! {
        img {
            class: "h-40",
            style: "{PIXEL_STYLE}",
            src: "/lostdisplay_peace.png"
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col justify-center items-center gap-4 w-dvw h-dvh bg-surface",
            Header {}
            div {
                class: "flex flex-row gap-8",
                LINKS.iter().map(|props| rsx!{ Link { url: props.url, icon: props.icon } })
            }
        }
    }
}
