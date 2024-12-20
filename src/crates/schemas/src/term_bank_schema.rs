#![cfg(target_os = "windows")]

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type StructuredContentData = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum FontStyle
{
    Normal,
    Italic
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum FontWeight
{
    Normal,
    Bold
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TextDecorationLine
{
    None,
    Underline,
    Overline,
    LineThrough,
    #[serde(untagged)]
    TextDecorationArray(Vec<TextDecorationLine>)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TextDecorationStyle
{
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlign
{
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TextAlign
{
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    JustifyAll,
    MatchParent
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum WordBreak
{
    Normal,
    BreakAll,
    KeepAll
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ImageRendering
{
    Auto,
    Pixelated,
    CrispEdges
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ImageAppearance
{
    Auto,
    Monochrome
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SizeUnit
{
    Px,
    Em
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredContentStyle
{
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_size: Option<String>,
    color: Option<String>,
    background: Option<String>,
    background_color: Option<String>,
    text_decoration_line: Option<TextDecorationLine>,
    text_decoration_style: Option<TextDecorationStyle>,
    text_decoration_color: Option<String>,
    border_color: Option<String>,
    border_style: Option<String>,
    border_radius: Option<String>,
    border_width: Option<String>,
    clip_path: Option<String>,
    vertical_align: Option<VerticalAlign>,
    text_align: Option<TextAlign>,
    text_emphasis: Option<String>,
    text_shadow: Option<String>,
    margin: Option<String>,
    margin_top: Option<String>,
    margin_left: Option<String>,
    margin_right: Option<String>,
    margin_bottom: Option<String>,
    padding: Option<String>,
    padding_top: Option<String>,
    padding_left: Option<String>,
    padding_right: Option<String>,
    padding_bottom: Option<String>,
    word_break: Option<WordBreak>,
    white_space: Option<String>,
    cursor: Option<String>,
    list_style_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyTag {
    tag: String,
    data: Option<StructuredContentNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericContainerTag {
    tag: String,
    content: Option<StructuredContentNode>,
    data: Option<StructuredContentData>,
    lang: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableTag {
    tag: String,
    content: Option<StructuredContentNode>,
    data: Option<StructuredContentData>,
    #[serde(rename = "colSpan")]
    col_span: Option<i32>,
    #[serde(rename = "rowSpan")]
    row_span: Option<i32>,
    style: Option<StructuredContentStyle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleContainerTag {
    tag: String,
    content: Option<StructuredContentNode>,
    data: Option<StructuredContentData>,
    style: Option<StructuredContentStyle>,
    title: Option<String>,
    lang: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageTag {
    tag: String,
    data: Option<StructuredContentData>,
    path: String,
    width: Option<i32>,
    height: Option<i32>,
    title: Option<String>,
    alt: Option<String>,
    description: Option<String>,
    pixelated: Option<bool>,
    image: Option<ImageRendering>,
    appearance: Option<ImageAppearance>,
    background: Option<bool>,
    collapsed: Option<bool>,
    collapsible: Option<bool>,
    #[serde(rename = "verticalAlign")]
    vertical_align: Option<VerticalAlign>,
    border: Option<String>,
    #[serde(rename = "borderRadius")]
    border_radius: Option<String>,
    size_units: Option<SizeUnit>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnchorTag {
    tag: String,
    content: Option<StructuredContentNode>,
    href: String,
    lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum TagVariant {
    #[serde(rename = "br")]
    Empty(EmptyTag),
    #[serde(rename = "ruby")]
    #[serde(alias = "rt")]
    #[serde(alias = "rp")]
    #[serde(alias = "table")]
    #[serde(alias = "thead")]
    #[serde(alias = "tbody")]
    #[serde(alias = "tfoot")]
    #[serde(alias = "tr")]
    GenericContainer(GenericContainerTag),
    #[serde(rename = "td")]
    #[serde(alias = "th")]
    Table(TableTag),
    #[serde(rename = "span")]
    #[serde(alias = "div")]
    #[serde(alias = "ol")]
    #[serde(alias = "ul")]
    #[serde(alias = "li")]
    #[serde(alias = "details")]
    #[serde(alias = "summary")]
    StyleContainer(StyleContainerTag),
    #[serde(rename = "img")]
    Image(ImageTag),
    #[serde(rename = "a")]
    Anchor(AnchorTag),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StructuredContentNode
{
    Variant(TagVariant),
    Text(String),
    ChildContent(Vec<StructuredContentNode>)
}

pub type StructuredContent = StructuredContentNode;
pub type DefinitionsObjectRaw = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedDefinitionText {
    #[serde(rename = "type")]
    dd_type: String,
    text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedDefinitionStructuredContent {
    #[serde(rename = "type")]
    dd_type: String,
    content: StructuredContent
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedDefinitionImage {
    #[serde(rename = "type")]
    dd_type: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum DetailedDefinitionVariant
{
    Text(DetailedDefinitionText),
    StructuredContent(StructuredContent),
    Image(DetailedDefinitionImage)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InflectionDefinition {
    uninflected_term: String,
    inflection_rules: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DetailedDefinition
{
    Variant(DetailedDefinitionVariant),
    Text(String),
    Inflection(InflectionDefinition)
}
pub type DefinitionsObject = Vec<StructuredContentNode>;

pub enum TermInformation
{
    Term(String),
    Reading(String),
    DefinitionTags(Option<String>),
    Deinflectors(String),
    Popularity(i32),
    Definitions(DefinitionsObjectRaw),
    SequenceNumber(i32),
    TermTags(String)
}

pub type DictionaryTermBankV3 = Vec<TermInformation>;
