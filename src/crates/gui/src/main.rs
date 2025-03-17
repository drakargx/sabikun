use std::{fs::File, io::BufReader};

use components::StructuredContent;
//use components::Hero;
use dioxus::prelude::*;
use dioxus_elements::{data, datalist};
use schemas::{
    DetailedDefinition, DictionaryTermBankV3, StructuredContentNode, StructuredContentStyle,
    StyledElement, TagElement, TermDefinition,
};
use serde::Deserialize;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    //let path = "C:\\code\\yomidb\\testzip\\single_term_bank_1.json";
    let path = "/home/drags/code/yomidb/testzip/single_term_bank_1.json";
    //let path = "C:\\code\\yomidb\\testzip\\term_bank_1.json";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let jd = &mut serde_json::Deserializer::from_reader(reader);
    let result: Result<DictionaryTermBankV3, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(ref some) => {
            let term = &some[0];
            match &term.definitions[0] {
                TermDefinition::Detailed(detailed) => match detailed {
                    DetailedDefinition::StructuredContent(node) => {
                        println!("{:?}", node);
                        rsx! {
                            StructuredContent { node: node.clone() }
                        }
                    }
                    _ => panic!("3"),
                },
                _ => panic!("2"),
            }
        }
        Err(err) => {
            panic!("Fail");
        }
    }
}
// #[component]
// fn App() -> Element {
//     // Build cool things ✌️
//     let mut attrs: Vec<Attribute> = vec![];
//     let tst = "sc-data-key";
//     let styled = StructuredContentNode::Variant(Box::new(TagElement::Styled(StyledElement {
//         tag: schemas::StyledElementTag::Li,
//         content: Some(StructuredContentNode::Text(String::from("repitition marker in katakana"))),
//         data: None,
//         style: Some(StructuredContentStyle { font_style: None, font_weight: None, font_size: None, color: None, background: None, background_color: None, text_decoration_line: None, text_decoration_style: None, text_decoration_color: None, border_color: None, border_style: None, border_radius: None, border_width: None, clip_path: None, vertical_align: None, text_align: None, text_emphasis: None, text_shadow: None, margin: None, margin_top: None, margin_left: None, margin_right: None, margin_bottom: None, padding: None, padding_top: None, padding_left: Some(String::from("0")), padding_right: None, padding_bottom: None, word_break: None, white_space: None, cursor: None, list_style_type: Some("none".to_string()) }),
//         title: None,
//         lang: None
//     })));
//     attrs.push(Attribute {name: "border-radius", value: dioxus_core::AttributeValue::Text(String::from("0.3em")), namespace: Some("style"), volatile: false});
//     attrs.push(Attribute {name: "margin-right", value: dioxus_core::AttributeValue::Text(String::from("0.25em")), namespace: Some("style"), volatile: false});
//     attrs.push(Attribute {name: "background-color", value: dioxus_core::AttributeValue::Text(String::from("color-mix(in srgb, #4a91ed 5%, transparent)")), namespace: Some("style"), volatile: false});
//     rsx! {
//         // Global app resources
//         //document::Link { rel: "icon", href: FAVICON }
//         //document::Link { rel: "stylesheet", href: MAIN_CSS }

//         div {
//             span {
//                 //"{tst}": "hello",
//                 font_size: "0.8em",
//                 font_weight: "bold",
//                 padding: "0.2em 0.3em",
//                 word_break: "keep-all",
//                 border_radius: "0.3em",
//                 vertical_align: "text-bottom",
//                 //background_color: "#565656",
//                 color: "white",
//                 cursor: "help",
//                 ..attrs,
//                 "unclass",
//             }
//             div {
//                 ul {
//                     li {
//                        list_style_type: "none", padding_left: "0",
//                        "repitition marker in katakana"
//                     }
//                     StructuredContent {node: styled}
//                 }
//                 div {
//                     margin_left: "0.5em",
//                     div {
//                         div {
//                             border_style: "none none none solid",
//                             padding: "0.5rem",
//                             border_radius: "0.4rem",
//                             margin_top: "0.5rem",
//                             margin_bottom: "0.5rem",
//                             border_color: "#4a91ed",
//                             background_color: "#4a91ed",
//                             //..attrs,
//                             div {
//                                 font_size: "1.3em",
//                                 span {
//                                     lang: "en",
//                                     font_size: "0.8em",
//                                     margin_right: "0.5rem",
//                                     "See:"
//                                 }
//                                 a {
//                                     lang: "ja",
//                                     href: "?query=一の字点&wildcards=off",
//                                     ruby {
//                                         "一",
//                                         rt {
//                                             "いち"
//                                         }
//                                     },
//                                     "の",
//                                     ruby {
//                                         "字",
//                                         rt {
//                                             "し"
//                                         }
//                                     },
//                                     ruby {
//                                         "点",
//                                         rt {
//                                             "てん"
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         //Hero {}

//     }
// }
