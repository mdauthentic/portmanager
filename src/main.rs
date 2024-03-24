#![allow(non_snake_case)]
use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use manganis::*;
mod app;
mod icon;
mod parser;

use crate::app::App;

fn main() {
    const _TAILWIND_URL: &str = mg!(file("public/tailwind.css"));

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("Port Manager")
                        .with_inner_size(LogicalSize::new(1280, 900))
                        .with_maximized(false),
                )
                .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()),
        )
        .launch(App)
}
