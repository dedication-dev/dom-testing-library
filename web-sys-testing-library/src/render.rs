use crate::dom::Element;
use web_sys::{DomParser, SupportedType};

/// Turns html into a [Element]
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
///     let _element = render_html("<div>Hello world!</div>");
/// }
/// ```
pub fn render_html(html: &str) -> Element {
    let web_sys_document: web_sys::Document = DomParser::new()
        .unwrap()
        .parse_from_string(
            r#" <!DOCTYPE html>
                <html>
                    <body>
                    </body>
                </html>"#,
            SupportedType::TextHtml,
        )
        .unwrap();

    let container = web_sys_document.create_element("div").unwrap();
    container.set_inner_html(html);

    Element::from(container)
}
