use dioxus::prelude::*;
use schemas::StructuredContentNode;
use schemas::StructuredContentStyle;
use schemas::TagElement;
use schemas::UnstyledElement;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

//just variant for now
#[component]
pub fn StructuredContent(node: StructuredContentNode) -> Element {
    match node {
        StructuredContentNode::Text(dat) => {
            rsx! {
                text {
                    {dat}
                }
            }
        }
        StructuredContentNode::Variant(elem) => build_sc_variant(elem),
        StructuredContentNode::ChildContent(vec) => {
            todo!()
        }
    }
}

fn build_sc_variant(variant: Box<TagElement>) -> Element {
    match *variant {
        TagElement::LineBreak(_) => {
            rsx! {
                br{
                    //TODO handle data field
                }
            }
        }
        TagElement::Unstyled(ref elem) => {
            rsx! {
                if elem.content.is_some() {
                    StructuredContent { node: elem.content.unwrap() }
                }
            }
        }
    }
}
