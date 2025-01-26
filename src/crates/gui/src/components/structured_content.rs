use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use schemas::*;

#[component]
pub fn StructuredContent(node: StructuredContentNode) -> Element {
    match node {
        StructuredContentNode::Text(dat) => {
            rsx! {
                {dat}
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

fn build_sc_variant(variant: Box<TagElement>) -> Element {
    match *variant {
        TagElement::Styled(ref elem) => sc_styled(elem),
        TagElement::LineBreak(ref elem) => sc_line_break(elem),
        TagElement::Unstyled(ref elem) => sc_unstyled(elem),
        TagElement::Table(ref elem) => sc_table(elem),
        TagElement::Image(ref elem) => sc_image(elem),
        TagElement::Link(ref elem) => sc_link(elem),
    }
}

macro_rules! push_attribute {
    ($attr:literal, $namespace:tt, $value:expr, $vec:expr) => {
        if let Some(val) = $value {
            $vec.push(Attribute {name: $attr, value: AttributeValue::Text(val.to_string()), namespace: $namespace, volatile: false})
        }
    }
}

fn style_into_vec(style: StructuredContentStyle) -> Vec<Attribute> {
    let mut attributes: Vec<Attribute> = vec![];
    let namespace = Some("style");

    push_attribute!("font-style", namespace, &style.font_style, attributes);
    push_attribute!("font-weight", namespace, &style.font_weight, attributes);
    push_attribute!("font-size", namespace, &style.font_size, attributes);
    push_attribute!("color", namespace, &style.color, attributes);
    push_attribute!("background", namespace, &style.background, attributes);
    push_attribute!("background-color", namespace, &style.background_color, attributes);
    push_attribute!("border-style", namespace, &style.border_style, attributes);
    push_attribute!("border-radius", namespace, &style.border_radius, attributes);
    push_attribute!("border-width", namespace, &style.border_width, attributes);
    push_attribute!("clip-path", namespace, &style.clip_path, attributes);
    push_attribute!("vertical-align", namespace, &style.vertical_align, attributes);
    push_attribute!("text_align", namespace, &style.text_align, attributes);
    push_attribute!("text-shadow", namespace, &style.text_shadow, attributes);
    push_attribute!("margin", namespace, &style.margin, attributes);
    push_attribute!("margin-top", namespace, &style.margin_top, attributes);
    push_attribute!("margin-left", namespace, &style.margin_left, attributes);
    push_attribute!("margin-right", namespace, &style.margin_right, attributes);
    push_attribute!("margin-bottom", namespace, &style.margin_bottom, attributes);
    push_attribute!("padding", namespace, &style.padding, attributes);
    push_attribute!("padding-top", namespace, &style.padding_top, attributes);
    push_attribute!("padding-left", namespace, &style.padding_left, attributes);
    push_attribute!("padding-right", namespace, &style.padding_right, attributes);
    push_attribute!("padding-bottom", namespace, &style.padding_bottom, attributes);
    push_attribute!("word-break", namespace, &style.word_break, attributes);
    push_attribute!("white-space", namespace, &style.white_space, attributes);
    push_attribute!("cursor", namespace, &style.cursor, attributes);
    push_attribute!("list-style-type", namespace, &style.list_style_type, attributes);

    attributes
}

//TODO data
//yomitan takes each key in data and adds an attribute with name "data-sc-{key}"
//borrow issue: attributes take a borrowed string as the name, so that name can't outlive where it came from
fn sc_line_break(elem: &LineBreakElement) -> Element {

    match elem.tag {
        LineBreakElementTag::Br => {
            rsx! {
                br {
                }
            }
        }
    }
}

pub fn sc_styled(elem: &StyledElement) -> Element {
    let style_attrs = style_into_vec(elem.style.clone().unwrap_or_default());
    let mut element_attrs: Vec<Attribute> = vec![];

    push_attribute!("title", None, elem.title.as_ref(), element_attrs);
    push_attribute!("lang", None, elem.lang.as_ref(), element_attrs);

    //TODO: think of a way to simplify this.
    //Can't use macros (I don't think), no way to convert elem.tag.to_string to a literal.
    match elem.tag {
        StyledElementTag::Span => {
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
        StyledElementTag::Div => {
            rsx! {
                div {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        StyledElementTag::Ol => {
            rsx! {
                ol {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        StyledElementTag::Ul => {
            rsx! {
                ul {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        StyledElementTag::Li => {
            rsx! {
                li {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        StyledElementTag::Details => {
            rsx! {
                details { 
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        StyledElementTag::Summary => {
            rsx! {
                summary {
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

fn sc_unstyled(elem: &UnstyledElement) -> Element {
    let mut element_attrs: Vec<Attribute> = vec![];

    push_attribute!("lang", None, elem.lang.as_ref(), element_attrs);
    match elem.tag {
        UnstyledElementTag::Ruby => {
            rsx! {
                ruby {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Rt => {
            rsx! {
                rt {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Rp => {
            rsx! {
                rp {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Table => {
            rsx! {
                table {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Thead => {
            rsx! {
                thead {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Tbody => {
            rsx! {
                tbody {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Tfoot => {
            rsx! {
                tfoot {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        },
        UnstyledElementTag::Tr => {
            rsx! {
                tr {
                    ..element_attrs,
                    if let Some(content) = &elem.content {
                        StructuredContent { node: content.clone() }
                    }
                }
            }
        }
    }
}

fn sc_table(elem: &TableElement) -> Element {
    let style_attrs = style_into_vec(elem.style.clone().unwrap_or_default());
    let mut element_attrs: Vec<Attribute> = vec![];

    push_attribute!("colspan", None, elem.col_span.as_ref(), element_attrs);
    push_attribute!("rowspan", None, elem.row_span.as_ref(), element_attrs);
    push_attribute!("lang", None, elem.lang.as_ref(), element_attrs);

    match elem.tag {
        TableElementTag::Td => {
            rsx! {
                td {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(node) = &elem.content {
                        StructuredContent { node: node.clone() }
                    }
                }
            }
        },
        TableElementTag::Th => {
            rsx! {
                th {
                    ..style_attrs,
                    ..element_attrs,
                    if let Some(node) = &elem.content {
                        StructuredContent { node: node.clone() }
                    }
                }
            }
        }
    }
}

fn sc_image(elem: &ImageElement) -> Element {
    let mut element_attrs: Vec<Attribute> = vec![];

    element_attrs.push(Attribute { name: "path", value: AttributeValue::Text(elem.path.clone()), namespace: None, volatile: false});
    //TODO data
    push_attribute!("width", None, elem.width, element_attrs);
    push_attribute!("height", None, elem.height, element_attrs);
    push_attribute!("title", None, &elem.title, element_attrs);
    push_attribute!("alt", None, &elem.alt, element_attrs);
    push_attribute!("description", None, &elem.description, element_attrs);
    push_attribute!("pixelated", None, elem.pixelated, element_attrs);
    push_attribute!("image", None, &elem.image, element_attrs);
    push_attribute!("apperance", None, &elem.appearance, element_attrs);
    push_attribute!("background", None, elem.background, element_attrs);
    push_attribute!("collapsed", None, elem.collapsed, element_attrs);
    push_attribute!("collapsible", None, elem.collapsible, element_attrs);
    push_attribute!("vertical-align", None, &elem.vertical_align, element_attrs);
    push_attribute!("border", None, &elem.border, element_attrs);
    push_attribute!("border-radius", None, &elem.border_radius, element_attrs);
    push_attribute!("size-units", None, &elem.size_units, element_attrs);

    match elem.tag {
        ImageElementTag::Img => {
            rsx! {
                img {
                    ..element_attrs,
                }
            }
        }
    }
}

fn sc_link(elem: &LinkElement) -> Element {
    let mut element_attrs: Vec<Attribute> = vec![];

    element_attrs.push(Attribute { name: "href", value: AttributeValue::Text(elem.href.clone()), namespace: None, volatile: false});
    //TODO data
    push_attribute!("lang", None, &elem.lang, element_attrs);

    match elem.tag {
        LinkElementTag::A => {
            rsx! {
                a {
                    ..element_attrs,
                    if let Some(node) = &elem.content {
                        StructuredContent { node: node.clone() }
                    }
                }
            }
        }
    }
}