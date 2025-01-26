use serde::de::value::MapAccessDeserializer;
use serde::de::{Deserializer, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use strum::Display;
use strum::EnumString;
use strum;
use strum_macros;

pub type StructuredContentData = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum FontWeight {
    Normal,
    Bold,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TextDecorationLine {
    None,
    Underline,
    Overline,
    LineThrough,
    #[serde(untagged)]
    TextDecorationArray(Vec<TextDecorationLine>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum VerticalAlign {
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    JustifyAll,
    MatchParent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ImageRendering {
    Auto,
    Pixelated,
    CrispEdges,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ImageAppearance {
    Auto,
    Monochrome,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum SizeUnit {
    Px,
    Em,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructuredContentStyle {
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<FontWeight>,
    pub font_size: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub background_color: Option<String>,
    pub text_decoration_line: Option<TextDecorationLine>,
    pub text_decoration_style: Option<TextDecorationStyle>,
    pub text_decoration_color: Option<String>,
    pub border_color: Option<String>,
    pub border_style: Option<String>,
    pub border_radius: Option<String>,
    pub border_width: Option<String>,
    pub clip_path: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
    pub text_align: Option<TextAlign>,
    pub text_emphasis: Option<String>,
    pub text_shadow: Option<String>,
    pub margin: Option<String>,
    pub margin_top: Option<String>,
    pub margin_left: Option<String>,
    pub margin_right: Option<String>,
    pub margin_bottom: Option<String>,
    pub padding: Option<String>,
    pub padding_top: Option<String>,
    pub padding_left: Option<String>,
    pub padding_right: Option<String>,
    pub padding_bottom: Option<String>,
    pub word_break: Option<WordBreak>,
    pub white_space: Option<String>,
    pub cursor: Option<String>,
    pub list_style_type: Option<String>,
}

//TODO each type of element should have its own tag enum?
//that way can generate errors in unlikely event new tags are added/tags moved to be unstyled

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum LineBreakElementTag {
    Br,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LineBreakElement {
    pub tag: LineBreakElementTag,
    pub data: Option<StructuredContentData>,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum UnstyledElementTag {
    Ruby,
    Rt,
    Rp,
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnstyledElement {
    pub tag: UnstyledElementTag,
    pub content: Option<StructuredContentNode>,
    pub data: Option<StructuredContentData>,
    pub lang: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum TableElementTag {
    Td,
    Th,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableElement {
    pub tag: TableElementTag,
    pub content: Option<StructuredContentNode>,
    pub data: Option<StructuredContentData>,
    pub col_span: Option<i32>,
    pub row_span: Option<i32>,
    pub style: Option<StructuredContentStyle>,
    pub lang: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum StyledElementTag {
    Span,
    Div,
    Ol,
    Ul,
    Li,
    Details,
    Summary,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyledElement {
    pub tag: StyledElementTag,
    pub content: Option<StructuredContentNode>,
    pub data: Option<StructuredContentData>,
    pub style: Option<StructuredContentStyle>,
    pub title: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum ImageElementTag {
    Img,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageElement {
    pub tag: ImageElementTag,
    pub data: Option<StructuredContentData>,
    pub path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub title: Option<String>,
    pub alt: Option<String>,
    pub description: Option<String>,
    pub pixelated: Option<bool>,
    pub image: Option<ImageRendering>,
    pub appearance: Option<ImageAppearance>,
    pub background: Option<bool>,
    pub collapsed: Option<bool>,
    pub collapsible: Option<bool>,
    pub vertical_align: Option<VerticalAlign>,
    pub border: Option<String>,
    pub border_radius: Option<String>,
    pub size_units: Option<SizeUnit>,
}

#[derive(Debug, PartialEq, Clone, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum LinkElementTag {
    A,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LinkElement {
    pub tag: LinkElementTag,
    pub content: Option<StructuredContentNode>,
    pub href: String,
    pub lang: Option<String>,
}

pub trait Tagged {
    fn tag(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum TagElement {
    LineBreak(LineBreakElement),
    Unstyled(UnstyledElement),
    Table(TableElement),
    Styled(StyledElement),
    Image(ImageElement),
    Link(LinkElement),
}

impl Tagged for TagElement {
    fn tag(&self) -> String {
        match *self {
            TagElement::LineBreak(ref elem) => elem.tag.to_string(),
            TagElement::Unstyled(ref elem) => elem.tag.to_string(),
            TagElement::Table(ref elem) => elem.tag.to_string(),
            TagElement::Styled(ref elem) => elem.tag.to_string(),
            TagElement::Image(ref elem) => elem.tag.to_string(),
            TagElement::Link(ref elem) => elem.tag.to_string(),
        }
    }
}

//TODO: think long and hard about TagVariant
//maybe:

/*
each TagVariant is its own struct
and they all implement the same trait that provides info on what tag they have

trait ContentTag { fn tag(&self) -> String }

and then instead of Variant(Box<TagVariant>) it could be Variant(Box<dyn ContentTag>)

and TagVariant could stay just for ease of deserializing, make it non-public


*/

#[derive(Debug, PartialEq, Clone)]
pub enum StructuredContentNode {
    Text(String),
    Variant(Box<TagElement>),
    ChildContent(Vec<StructuredContentNode>),
}

impl Serialize for StructuredContentNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StructuredContentNode::Text(ref s) => serializer.serialize_str(s),
            StructuredContentNode::Variant(ref elem) => serialize_node_variant(elem.as_ref(), serializer),
            StructuredContentNode::ChildContent(ref v) => {
                let mut seq = serializer.serialize_seq(None)?;
                for children in v.iter() {
                    seq.serialize_element(children)?;
                }
                seq.end()
            }
        }
    }
}

fn serialize_node_variant<A>(node: &TagElement, serializer: A) -> Result<A::Ok, A::Error>
where
    A: Serializer,
{
    //If option is Some, unwrap it and serialize it to the map.
    //Otherwise, don't serialize it to keep json compact.
    macro_rules! serialize_opt {
        ($e:expr, $name:literal, $m:expr) => {
            if $e.is_some() {
                $m.serialize_entry($name, &$e.as_ref().unwrap())?;
            }
        }
    }
    fn serialize_line_break<A>(elem: &LineBreakElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        serialize_opt!(elem.data, "data", map);
        map.end()
    }

    fn serialize_unstyled<A>(elem: &UnstyledElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        serialize_opt!(elem.data, "data", map);
        serialize_opt!(elem.lang, "lang", map);
        serialize_opt!(elem.content, "content", map);
        map.end()
    }

    fn serialize_table<A>(elem: &TableElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        serialize_opt!(elem.data, "data", map);
        serialize_opt!(elem.col_span, "colSpan", map);
        serialize_opt!(elem.row_span, "rowSpan", map);
        serialize_opt!(elem.style, "style", map);
        serialize_opt!(elem.lang, "lang", map);
        serialize_opt!(elem.content, "content", map);
        map.end()
    }

    fn serialize_styled<A>(elem: &StyledElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        serialize_opt!(elem.data, "data", map);
        serialize_opt!(elem.style, "style", map);
        serialize_opt!(elem.title, "title", map);
        serialize_opt!(elem.lang, "lang", map);
        serialize_opt!(elem.content, "content", map);
        map.end()
    }

    fn serialize_image<A>(elem: &ImageElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        map.serialize_entry("path", &elem.path)?;

        serialize_opt!(elem.data, "data", map);
        serialize_opt!(elem.width, "width", map);
        serialize_opt!(elem.height, "height", map);
        serialize_opt!(elem.title, "title", map);
        serialize_opt!(elem.alt, "alt", map);
        serialize_opt!(elem.description, "description", map);
        serialize_opt!(elem.pixelated, "pixelated", map);
        serialize_opt!(elem.image, "imageRendering", map);
        serialize_opt!(elem.appearance, "appearance", map);
        serialize_opt!(elem.background, "background", map);
        serialize_opt!(elem.collapsed, "collapsed", map);
        serialize_opt!(elem.collapsible, "collapsible", map);
        serialize_opt!(elem.vertical_align, "verticalAlign", map);
        serialize_opt!(elem.border, "border", map);
        serialize_opt!(elem.border_radius, "borderRadius", map);
        serialize_opt!(elem.size_units, "sizeUnits", map);

        map.end()
    }

    fn serialize_link<A>(elem: &LinkElement, mut map: A) -> Result<A::Ok, A::Error>
    where
        A: SerializeMap,
    {
        map.serialize_entry("href", &elem.href)?;

        serialize_opt!(elem.lang, "lang", map);
        serialize_opt!(elem.content, "content", map);
        map.end()
    }

    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry("tag", node.tag().as_str())?;
    match node {
        TagElement::LineBreak(ref elem) => serialize_line_break(elem, map),
        TagElement::Unstyled(ref elem) => serialize_unstyled(elem, map),
        TagElement::Table(ref elem) => serialize_table(elem, map),
        TagElement::Styled(ref elem) => serialize_styled(elem, map),
        TagElement::Image(ref elem) => serialize_image(elem, map),
        TagElement::Link(ref elem) => serialize_link(elem, map),
    }
}

impl<'de> Deserialize<'de> for StructuredContentNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = StructuredContentNode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("structured content node")
            }

            fn visit_str<S>(self, text: &str) -> Result<Self::Value, S>
            where
                S: Error,
            {
                Ok(StructuredContentNode::Text(text.to_string()))
            }

            fn visit_seq<S>(self, mut sequence: S) -> Result<StructuredContentNode, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut children = Vec::new();

                while let Some(item) = sequence.next_element::<StructuredContentNode>()? {
                    children.push(item);
                }

                Ok(StructuredContentNode::ChildContent(children))
            }

            fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Ok(StructuredContentNode::Variant(Box::new(
                    deserialize_node_variant(access)?,
                )))
            }
        }

        deserializer.deserialize_any(NodeVisitor)
    }
}

fn deserialize_node_variant<'de, A>(mut access: A) -> Result<TagElement, A::Error>
where
    A: MapAccess<'de>,
{
    fn deserialize_empty_tag<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        const TYPE_FIELDS: &[&str] = &["data"];
        let mut data = None;

        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "data" => {
                    data = Some(access.next_value::<StructuredContentData>()?);
                    //result = TagVariant::Empty{ tag, data: Some(access.next_value::<StructuredContentData>()?)};
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        Ok(TagElement::LineBreak(LineBreakElement { 
            tag: LineBreakElementTag::from_str(tag).unwrap(),
            data
        }))
    }

    fn deserialize_generic_container<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        const TYPE_FIELDS: &[&str] = &["content", "data", "lang"];

        let mut content = None;
        let mut data = None;
        let mut lang = None;
        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "content" => {
                    content = Some(access.next_value::<StructuredContentNode>()?);
                }
                "data" => {
                    data = Some(access.next_value::<StructuredContentData>()?);
                }
                "lang" => {
                    lang = Some(access.next_value::<String>()?);
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        Ok(TagElement::Unstyled(UnstyledElement {
            tag: UnstyledElementTag::from_str(tag).unwrap(),
            content,
            data,
            lang,
        }))
    }

    fn deserialize_table<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        const TYPE_FIELDS: &[&str] = &["content", "data", "colSpan", "rowSpan", "style", "lang"];

        let mut content = None;
        let mut data = None;
        let mut col_span = None;
        let mut row_span = None;
        let mut style = None;
        let mut lang = None;

        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "content" => {
                    content = Some(access.next_value::<StructuredContentNode>()?);
                }
                "data" => {
                    data = Some(access.next_value::<StructuredContentData>()?);
                }
                "colSpan" => {
                    col_span = Some(access.next_value::<i32>()?);
                }
                "rowSpan" => {
                    row_span = Some(access.next_value::<i32>()?);
                }
                "style" => {
                    style = Some(access.next_value::<StructuredContentStyle>()?);
                }
                "lang" => {
                    lang = Some(access.next_value::<String>()?);
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        Ok(TagElement::Table(TableElement {
            tag: TableElementTag::from_str(tag).unwrap(),
            content,
            data,
            col_span,
            row_span,
            style,
            lang,
        }))
    }

    fn deserialize_style_container<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        const TYPE_FIELDS: &[&str] = &["content", "data", "style", "title", "lang"];

        let mut content = None;
        let mut data = None;
        let mut style = None;
        let mut title = None;
        let mut lang = None;

        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "content" => {
                    content = Some(access.next_value::<StructuredContentNode>()?);
                }
                "data" => {
                    data = Some(access.next_value::<StructuredContentData>()?);
                }
                "style" => {
                    style = Some(access.next_value::<StructuredContentStyle>()?);
                }
                "title" => {
                    title = Some(access.next_value::<String>()?);
                }
                "lang" => {
                    lang = Some(access.next_value::<String>()?);
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        Ok(TagElement::Styled(StyledElement {
            tag: StyledElementTag::from_str(tag).unwrap(),
            content,
            data,
            style,
            title,
            lang,
        }))
    }

    fn deserialize_image<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        //has required "path" field as well
        const TYPE_FIELDS: &[&str] = &["content", "data", "style", "title", "lang"];

        let mut data = None;
        let mut path_opt = None;
        let mut width = None;
        let mut height = None;
        let mut title = None;
        let mut alt = None;
        let mut description = None;
        let mut pixelated = None;
        let mut image = None;
        let mut appearance = None;
        let mut background = None;
        let mut collapsed = None;
        let mut collapsible = None;
        let mut vertical_align = None;
        let mut border = None;
        let mut border_radius = None;
        let mut size_units = None;

        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "data" => {
                    data = Some(access.next_value::<StructuredContentData>()?);
                }
                "path" => {
                    path_opt = Some(access.next_value::<String>()?);
                }
                "width" => {
                    width = Some(access.next_value::<i32>()?);
                }
                "height" => {
                    height = Some(access.next_value::<i32>()?);
                }
                "title" => {
                    title = Some(access.next_value::<String>()?);
                }
                "alt" => {
                    alt = Some(access.next_value::<String>()?);
                }
                "description" => {
                    description = Some(access.next_value::<String>()?);
                }
                "pixelated" => {
                    pixelated = Some(access.next_value::<bool>()?);
                }
                "image" => {
                    image = Some(access.next_value::<ImageRendering>()?);
                }
                "appearance" => {
                    appearance = Some(access.next_value::<ImageAppearance>()?);
                }
                "background" => {
                    background = Some(access.next_value::<bool>()?);
                }
                "collapsed" => {
                    collapsed = Some(access.next_value::<bool>()?);
                }
                "collapsible" => {
                    collapsible = Some(access.next_value::<bool>()?);
                }
                "verticalAlign" => {
                    vertical_align = Some(access.next_value::<VerticalAlign>()?);
                }
                "border" => {
                    border = Some(access.next_value::<String>()?);
                }
                "borderRadius" => {
                    border_radius = Some(access.next_value::<String>()?);
                }
                "sizeUnits" => {
                    size_units = Some(access.next_value::<SizeUnit>()?);
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        if path_opt.is_none() {
            Err(<A::Error as Error>::missing_field("path"))
        } else {
            let path = path_opt.unwrap();
            Ok(TagElement::Image(ImageElement {
                tag: ImageElementTag::from_str(tag).unwrap(),
                data,
                path,
                width,
                height,
                title,
                alt,
                description,
                pixelated,
                image,
                appearance,
                background,
                collapsed,
                collapsible,
                vertical_align,
                border,
                border_radius,
                size_units,
            }))
        }
    }

    fn deserialize_link<'de, A>(mut access: A, tag: &str) -> Result<TagElement, A::Error>
    where
        A: MapAccess<'de>,
    {
        //has required "href" field as well
        const TYPE_FIELDS: &[&str] = &["content", "href", "lang"];

        let mut content = None;
        let mut href_opt = None;
        let mut lang = None;

        while let Some(key) = access.next_key::<String>()? {
            let value = key.as_ref();
            match value {
                "content" => {
                    content = Some(access.next_value::<StructuredContentNode>()?);
                }
                "href" => {
                    href_opt = Some(access.next_value::<String>()?);
                }
                "lang" => {
                    lang = Some(access.next_value::<String>()?);
                }
                _ => return Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
            }
        }

        if href_opt.is_none() {
            Err(<A::Error as Error>::missing_field("href"))
        } else {
            let href = href_opt.unwrap();
            Ok(TagElement::Link(LinkElement {
                tag: LinkElementTag::from_str(tag).unwrap(),
                content,
                href,
                lang,
            }))
        }
    }

    const TYPE_FIELDS: &[&str] = &["tag"];

    //first key should always be "tag"
    if Some("tag") != access.next_key::<String>()?.as_deref() {
        return Err(<A::Error as Error>::missing_field("tag"));
    }

    let binding = access.next_value::<String>()?;
    let value = binding.as_ref();

    match value {
        "br" => deserialize_empty_tag(access, value),
        "ruby" | "rt" | "rp" | "table" | "thead" | "tbody" | "tfoot" | "tr" => {
            deserialize_generic_container(access, value)
        }
        "td" | "th" => deserialize_table(access, value),
        "span" | "div" | "ol" | "ul" | "li" | "details" | "summary" => {
            deserialize_style_container(access, value)
        }
        "img" => deserialize_image(access, value),
        "a" => deserialize_link(access, value),
        _ => Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TypedImage {
    path: String,
    width: Option<i32>,
    height: Option<i32>,
    title: Option<String>,
    alt: Option<String>,
    description: Option<String>,
    pixelated: Option<bool>,
    #[serde(rename = "imageRendering")]
    image_rendering: Option<ImageRendering>,
    appearance: Option<ImageAppearance>,
    background: Option<bool>,
    collapsed: Option<bool>,
    collapsible: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Deinflection {
    uninflected_term: String,
    inflection_rules: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DetailedDefinition {
    StructuredContent(StructuredContentNode),
    Text(String),
    Image(TypedImage),
}

#[derive(Debug)]
pub enum TermDefinition {
    Simple(String),
    Detailed(DetailedDefinition),
    Inflection(Deinflection),
}
//NOTE this needs to return a Vec
impl Serialize for TermDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TermDefinition::Simple(ref s) => {
                let seq = serializer.serialize_str(s.as_ref());
                seq
            }
            TermDefinition::Detailed(ref dd) => match dd {
                DetailedDefinition::Text(s) => {
                    let mut seq = serializer.serialize_map(None)?;
                    seq.serialize_entry("type", "text")?;
                    seq.serialize_entry("text", s)?;
                    seq.end()
                }
                DetailedDefinition::Image(img) => {
                    let mut seq = serializer.serialize_map(None)?;
                    seq.serialize_entry("type", "image")?;
                    seq.serialize_entry("image", img)?;
                    seq.end()
                }
                DetailedDefinition::StructuredContent(sc) => {
                    let mut seq = serializer.serialize_map(None)?;
                    seq.serialize_entry("type", "structured-content")?;
                    seq.serialize_entry("content", sc)?;
                    seq.end()
                }
            },
            TermDefinition::Inflection(ref inflect) => {
                let mut seq = serializer.serialize_map(Some(2))?;
                seq.serialize_entry::<str, String>("uninflected_term", &inflect.uninflected_term)?;
                seq.serialize_entry::<str, Vec<String>>(
                    "inflection_rules",
                    &inflect.inflection_rules,
                )?;
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for TermDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TermVisitor;

        impl<'de> Visitor<'de> for TermVisitor {
            type Value = TermDefinition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or detailed definition")
            }

            fn visit_str<S>(self, text: &str) -> Result<Self::Value, S>
            where
                S: Error,
            {
                Ok(TermDefinition::Simple(text.to_string()))
            }

            //structured
            fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Ok(TermDefinition::Detailed(deserialize_detailed(access)?))
            }

            //deinflection
            fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                // there should be 2 elements, one is a string, the second is Vec<String>
                let term = access.next_element::<String>()?;
                let rules = access.next_element::<Vec<String>>()?;

                match access.next_element::<IgnoredAny>() {
                    Ok(None) => (),
                    _ => {
                        return Err(<S::Error as Error>::invalid_length(
                            2,
                            &"an array with 2 elements",
                        ))
                    }
                }

                //check term and rules
                if term.is_none() {
                    return Err(<S::Error as Error>::missing_field("inflected term"));
                }
                if rules.is_none() {
                    return Err(<S::Error as Error>::missing_field("inflection rules"));
                }

                Ok(TermDefinition::Inflection(Deinflection {
                    uninflected_term: term.unwrap(),
                    inflection_rules: rules.unwrap(),
                }))
            }
        }

        deserializer.deserialize_any(TermVisitor)
    }
}

fn deserialize_detailed<'de, A>(mut access: A) -> Result<DetailedDefinition, A::Error>
where
    A: MapAccess<'de>,
{
    const TYPE_FIELDS: &[&str] = &["structured-content", "image", "text"];

    //first key should always be "type"
    if Some("type") != access.next_key::<String>()?.as_deref() {
        return Err(<A::Error as Error>::missing_field("type"));
    }

    let binding = access.next_value::<String>()?;
    let value = binding.as_ref();

    match value {
        "structured-content" => Ok(DetailedDefinition::StructuredContent(
            deserialize_structured_content(access)?,
        )),
        "text" => Ok(DetailedDefinition::Text(access.next_value::<String>()?)),
        "image" => Ok(DetailedDefinition::Image(TypedImage::deserialize(
            MapAccessDeserializer::new(access),
        )?)),
        _ => Err(<A::Error as Error>::unknown_variant(value, TYPE_FIELDS)),
    }
}

fn deserialize_structured_content<'de, A>(mut access: A) -> Result<StructuredContentNode, A::Error>
where
    A: MapAccess<'de>,
{
    //needs to have a "content" key
    if Some("content") != access.next_key::<String>()?.as_deref() {
        println!("could not find content key");
        return Err(<A::Error as Error>::missing_field("content"));
    }

    access.next_value::<StructuredContentNode>()
}

//for a definition, it can be one of the following:
// - String
// - an object with a field "type"
//      - this "type" field can be "structured-content", "image", or "text"
//          - if type is "structured-content", it has another field "content" that is a node
//          - if type is "text", it has another field "text" that is string
//          - if type is "image", it has another field "path", along with some optional properties
// and all of these are wrapped in an array
#[derive(Debug)]
pub struct TermInformation {
    pub term: String,
    pub reading: String,
    pub definition_tags: Option<String>,
    pub deinflectors: String,
    pub popularity: i32,
    pub definitions: Vec<TermDefinition>,
    pub sequence_number: i32,
    pub term_tags: String,
}

#[derive(Serialize, Deserialize)]
struct TermInfoArray(
    String,
    String,
    Option<String>,
    String,
    i32,
    Vec<TermDefinition>,
    i32,
    String,
);

impl<'de> Deserialize<'de> for TermInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let TermInfoArray(
            term,
            reading,
            definition_tags,
            deinflectors,
            popularity,
            definitions,
            sequence_number,
            term_tags,
        ) = TermInfoArray::deserialize(deserializer)?;

        Ok(TermInformation {
            term,
            reading,
            definition_tags,
            deinflectors,
            popularity,
            definitions,
            sequence_number,
            term_tags,
        })
    }
}

impl Serialize for TermInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //TODO change to constant
        let mut seq = serializer.serialize_seq(Some(8))?;
        seq.serialize_element(&self.term)?;
        seq.serialize_element(&self.reading)?;
        seq.serialize_element(&self.definition_tags)?;
        seq.serialize_element(&self.deinflectors)?;
        seq.serialize_element(&self.popularity)?;
        seq.serialize_element(&self.definitions)?;
        seq.serialize_element(&self.sequence_number)?;
        seq.serialize_element(&self.term_tags)?;
        seq.end()
    }
}

pub type DictionaryTermBankV3 = Vec<TermInformation>;
