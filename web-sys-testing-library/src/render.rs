use crate::dom::document::Document;
use web_sys::{DomParser, SupportedType};

pub fn render(html: &str) -> Document {
    Document::from(parse_document(html))
}

fn parse_document(html: &str) -> web_sys::Document {
    DomParser::new()
        .unwrap()
        .parse_from_string(html, SupportedType::TextHtml)
        .unwrap()
}
