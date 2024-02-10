#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

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

fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    render! {
        a {
            href: "{cx.props.url}",
            img {
                class: "h-20",
                style: "image-rendering: crisp-edges",
                src: "/{cx.props.icon}"
            }
        }
    }
}

const LINKS: [LinkProps<'static>; 4] = [
    LinkProps::new("itch.png", "https://itch.io/lostdisplay"),
    LinkProps::new("youtube.png", "https://youtube.com/lostdisplay"),
    LinkProps::new("mastodon.png", "https://mastodon.gamedev.place/@lostdisplay"),
    LinkProps::new("kofi.png", "https://ko-fi.com/lostdisplay"),
];

fn Header(cx: Scope) -> Element {
    render! {
        img {
            class: "h-48 w-48",
            style: "image-rendering: crisp-edges",
            src: "/lostdisplay_peace.png"
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col justify-center items-center gap-2 w-dvw h-dvh bg-surface",
            Header {}
            div {
                class: "flex flex-row",
                LINKS.iter().map(|props| rsx!{ Link { url: props.url, icon: props.icon } })
            }
        }
    }
}
