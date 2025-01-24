#![cfg(target_os = "windows")]
 
use dioxus::prelude::*;
use schemas::FontStyle;
use schemas::StructuredContentElement;
use schemas::StructuredContentNode;
use schemas::StructuredContentStyle;
use schemas::StyledElement;
use schemas::TagElement;
use dioxus_core::AttributeValue;
use schemas::UnstyledElement;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn style_into_vec(style: StructuredContentStyle) -> Vec<Attribute> {
    let mut attributes: Vec<Attribute> = vec![];

    if let Some(font_style) = &style.font_style {
        attributes.push(Attribute {name: "font-style", value: AttributeValue::Text(font_style.to_string()), namespace: Some("style"), volatile: false});
    }

    attributes
}

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
            rsx! {
                for elem in vec {
                    StructuredContent { node: elem }
                }
            }
        }
    }
}

/*
   MESSY!!!

   each element might allow children, and might have style
   for style, it would be great if there was a way that all elements could be checked,
   that way can handle their attributes before the match

   ...might need a macro
*/
fn build_sc_variant(variant: Box<TagElement>) -> Element {
    let attributes = style_into_vec(variant.style().unwrap_or_default());
    //extract all the variables out of style here?
    //or impl Display trait for style struct and then set style like that
    //style
    // match *variant {
    //     TagElement::Styled(ref elem) => sc_styled(elem),
    //     TagElement::LineBreak(ref line_break_element) => todo!(),
    //     TagElement::Unstyled(ref unstyled_element) => todo!(),
    //     TagElement::Table(ref table_element) => todo!(),
    //     TagElement::Image(ref image_element) => todo!(),
    //     TagElement::Link(ref link_element) => todo!(),
    // }
}

macro_rules! push_attribute {
    ($attr:literal, $namespace:tt, $value:expr, $vec:expr) => {
        if $value.is_some() {
            $vec.push(Attribute {name: $attr, value: AttributeValue::Text($value.unwrap()), namespace: $namespace, volatile: false})
        }
    }
}

fn sc_styled(elem: &StyledElement) -> Element {
    let style_attrs = style_into_vec(elem.style.clone().unwrap_or_default());
    let mut element_attrs: Vec<Attribute> = vec![];

    push_attribute!("title", None, elem.title, element_attrs);
    push_attribute!("lang", None, elem.lang, element_attrs);
    
    match elem.tag.as_ref() {
        "span" => {
            rsx! {
                span {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        
    }
    
}
