use web_sys::{Document, DomParser, SupportedType};

pub trait ParseDocument {
    fn parse_document(&self) -> Document;
}

impl ParseDocument for &str {
    fn parse_document(&self) -> Document {
        DomParser::new()
            .unwrap()
            .parse_from_string(self, SupportedType::TextHtml)
            .unwrap()
    }
}
