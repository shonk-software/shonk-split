use std::cell::Ref;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use dioxus::html::a::position;
use dioxus::prelude::*;
use tracing::info;

use shonk_split_model::Position;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    let mut positions = use_resource(|| async move {
        reqwest::get("http://localhost:8080/rewe_data")
            .await
            .unwrap()
            .json::<Vec<Position>>()
            .await
            .unwrap()
    });

    // let positions = positions.read_unchecked().unwrap();

    // rsx! {
    //     Testest {}

        // for position in positions {
        //
        // }
        // match &*positions.read_unchecked() {
        //     Some(response)=> rsx!{
        //         "hello full"
        //     },
        //     None => rsx! {
        //         "hello empty"
        //     },
        // }

    // }
    rsx! {
        div { class: "bg-red-100",
            button {
                onclick: move |_| info!("Clicked"),
                "Click me!"
            }
        }
        br {}

        p {"Dragons are cool!"} br {}

        match &*positions.read_unchecked() {
            Some(response)=> rsx!{
                ul {
                    for item in response {
                        li {
                            "{item:?}"
                        }
                    }
                }
            },
            None => rsx! {
                "hello empty"
            },
        }
    }
}
