use web_sys::{DomParser, SupportedType};

pub fn render_html(html: &str) -> web_sys::Document {
    DomParser::new()
        .unwrap()
        .parse_from_string(html, SupportedType::TextHtml)
        .unwrap()
}
