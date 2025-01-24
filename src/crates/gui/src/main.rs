//use components::Hero;
use dioxus::prelude::*;
use dioxus_elements::{data, datalist};
use schemas::DictionaryTermBankV3;
use serde::Deserialize;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut attrs: Vec<Attribute> = vec![];
    attrs.push(Attribute {name: "border-radius", value: dioxus_core::AttributeValue::Text(String::from("0.3em")), namespace: Some("style"), volatile: false});
    attrs.push(Attribute {name: "margin-right", value: dioxus_core::AttributeValue::Text(String::from("0.25em")), namespace: Some("style"), volatile: false});
    attrs.push(Attribute {name: "background-color", value: dioxus_core::AttributeValue::Text(String::from("color-mix(in srgb, #4a91ed 5%, transparent)")), namespace: Some("style"), volatile: false});
    rsx! {
        // Global app resources
        //document::Link { rel: "icon", href: FAVICON }
        //document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            span {
                font_size: "0.8em",
                font_weight: "bold",
                padding: "0.2em 0.3em",
                word_break: "keep-all",
                border_radius: "0.3em",
                vertical_align: "text-bottom",
                //background_color: "#565656",
                color: "white",
                cursor: "help",
                ..attrs,
                "unclass",
            }
            div {
                ul {
                    li {
                        list_style_type: "none", padding_left: "0",
                        "repitition marker in katakana"
                    }
                }
                div {
                    margin_left: "0.5em",
                    div {
                        div {
                            border_style: "none none none solid",
                            padding: "0.5rem",
                            border_radius: "0.4rem",
                            margin_top: "0.5rem",
                            margin_bottom: "0.5rem",
                            border_color: "#4a91ed",
                            //background_color: "#4a91ed",
                            ..attrs,
                            div {
                                font_size: "1.3em",
                                span {
                                    lang: "en",
                                    font_size: "0.8em",
                                    margin_right: "0.5rem",
                                    "See:"
                                }
                                a {
                                    lang: "ja",
                                    href: "?query=一の字点&wildcards=off",
                                    ruby {
                                        "一",
                                        rt {
                                            "いち"
                                        }
                                    },
                                    "の",
                                    ruby {
                                        "字",
                                        rt {
                                            "し"
                                        }
                                    },
                                    ruby {
                                        "点",
                                        rt {
                                            "てん"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //Hero {}

    }
}
