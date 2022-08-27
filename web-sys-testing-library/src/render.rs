use web_sys::{DomParser, SupportedType};

/// Turns html into a [web_sys::Document]
///
/// # Panics
///
/// Panics when run on non-wasm targets
///
/// # Examples
/// ```no_run
/// use wasm_bindgen_test::*;
/// use web_sys_testing_library::render_html;
///
/// wasm_bindgen_test_configure!(run_in_browser);
///
/// #[wasm_bindgen_test]
/// fn test() {
///     let _document = render_html("<div>Hello world!</div>");
/// }
/// ```
pub fn render_html(html: &str) -> web_sys::Document {
    DomParser::new()
        .unwrap()
        .parse_from_string(html, SupportedType::TextHtml)
        .unwrap()
}
