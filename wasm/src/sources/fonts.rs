use crate::utils::dom::with_iframe;
use wasm_bindgen::JsValue;

// We use m or w because these two characters take up the maximum width.
// And we use a LLi so that the same matching fonts can get separated.
const TEST_STRING: &str = "mmMwWLliI0O&1";

// We test using 48px font size, we may use any size. I guess larger the better.
const TEXT_SIZE: &str = "48px";

// A font will be compared against all the three default fonts.
// And if for any default fonts it doesn't match, then that font is available.
const BASE_FONTS: &[&str] = &["monospace", "sans-serif", "serif"];

const FONT_LIST: &[&str] = &[
    // This is android-specific font from "Roboto" family
    "sans-serif-thin",
    "ARNO PRO",
    "Agency FB",
    "Arabic Typesetting",
    "Arial Unicode MS",
    "AvantGarde Bk BT",
    "BankGothic Md BT",
    "Batang",
    "Bitstream Vera Sans Mono",
    "Calibri",
    "Century",
    "Century Gothic",
    "Clarendon",
    "EUROSTILE",
    "Franklin Gothic",
    "Futura Bk BT",
    "Futura Md BT",
    "GOTHAM",
    "Gill Sans",
    "HELV",
    "Haettenschweiler",
    "Helvetica Neue",
    "Humanst521 BT",
    "Leelawadee",
    "Letter Gothic",
    "Levenim MT",
    "Lucida Bright",
    "Lucida Sans",
    "Menlo",
    "MS Mincho",
    "MS Outlook",
    "MS Reference Specialty",
    "MS UI Gothic",
    "MT Extra",
    "MYRIAD PRO",
    "Marlett",
    "Meiryo UI",
    "Microsoft Uighur",
    "Minion Pro",
    "Monotype Corsiva",
    "PMingLiU",
    "Pristina",
    "SCRIPTINA",
    "Segoe UI Light",
    "Serifa",
    "SimHei",
    "Small Fonts",
    "Staccato222 BT",
    "TRAJAN PRO",
    "Univers CE 55 Medium",
    "Vrinda",
    "ZWAdobeF",
];

// kudos to http://www.lalit.org/lab/javascript-css-font-detect/
pub async fn get_fonts() -> Result<JsValue, JsValue> {
    // Running the script in an iframe makes it not affect the page look and not be affected by the
    // page CSS. See: https://github.com/fingerprintjs/fingerprintjs/issues/592
    // https://github.com/fingerprintjs/fingerprintjs/issues/628
    let available_fonts = with_iframe(|_iframe, i_window| {
        let document = match i_window.document() {
            Some(doc) => doc,
            None => return Vec::new(),
        };

        let holder = match document.body() {
            Some(body) => body,
            None => return Vec::new(),
        };

        let _ = holder.style().set_property("font-size", TEXT_SIZE);

        // div to load spans for the default fonts and the fonts to detect
        let spans_container = match document.create_element("div") {
            Ok(el) => el,
            Err(_) => return Vec::new(),
        };

        let _ = spans_container.set_attribute(
            "style",
            "visibility: hidden; position: absolute; top: 0; left: 0;",
        );

        let create_span = |font_family: &str| -> Option<web_sys::Element> {
            let span = document.create_element("span").ok()?;
            let _ = span.set_attribute(
                "style",
                &format!(
                    "position: absolute; top: 0; left: 0; font-family: {};",
                    font_family
                ),
            );
            span.set_text_content(Some(TEST_STRING));
            let _ = spans_container.append_child(&span);
            Some(span)
        };

        // creates spans for the base fonts and adds them to baseFontsDiv
        let base_fonts_spans: Vec<web_sys::Element> = BASE_FONTS
            .iter()
            .filter_map(|&font| create_span(font))
            .collect();

        // creates spans for the fonts to detect and adds them to fontsDiv
        // Stores {fontName : [spans for that font]}
        let fonts_spans: std::collections::HashMap<String, Vec<web_sys::Element>> = FONT_LIST
            .iter()
            .map(|&font| {
                let spans: Vec<web_sys::Element> = BASE_FONTS
                    .iter()
                    .filter_map(|&base_font| create_span(&format!("'{}',{}", font, base_font)))
                    .collect();
                (font.to_string(), spans)
            })
            .collect();

        // add all the spans to the DOM
        let _ = holder.append_child(&spans_container);

        // get the default width for the three base fonts
        let mut default_width: std::collections::HashMap<&str, i32> =
            std::collections::HashMap::new();
        let mut default_height: std::collections::HashMap<&str, i32> =
            std::collections::HashMap::new();

        for (i, &base_font) in BASE_FONTS.iter().enumerate() {
            if i < base_fonts_spans.len() {
                let span = &base_fonts_spans[i];
                let w = js_sys::Reflect::get(span, &JsValue::from_str("offsetWidth"))
                    .ok()
                    .and_then(|v| v.as_f64())
                    .map(|v| v as i32)
                    .unwrap_or(0);
                let h = js_sys::Reflect::get(span, &JsValue::from_str("offsetHeight"))
                    .ok()
                    .and_then(|v| v.as_f64())
                    .map(|v| v as i32)
                    .unwrap_or(0);
                default_width.insert(base_font, w);
                default_height.insert(base_font, h);
            }
        }

        // check available fonts
        let mut available_fonts = Vec::new();

        for &font in FONT_LIST {
            if let Some(spans) = fonts_spans.get(font) {
                // checks if a font is available
                let is_available = BASE_FONTS.iter().enumerate().any(|(i, &base_font)| {
                    if i < spans.len() {
                        let span = &spans[i];
                        let w = js_sys::Reflect::get(span, &JsValue::from_str("offsetWidth"))
                            .ok()
                            .and_then(|v| v.as_f64())
                            .map(|v| v as i32)
                            .unwrap_or(0);
                        let h = js_sys::Reflect::get(span, &JsValue::from_str("offsetHeight"))
                            .ok()
                            .and_then(|v| v.as_f64())
                            .map(|v| v as i32)
                            .unwrap_or(0);
                        w != default_width[base_font] || h != default_height[base_font]
                    } else {
                        false
                    }
                });

                if is_available {
                    available_fonts.push(font);
                }
            }
        }

        available_fonts
    })
    .await?;

    let result = js_sys::Array::new();
    for font in available_fonts {
        result.push(&JsValue::from_str(font));
    }

    Ok(JsValue::from(result))
}
