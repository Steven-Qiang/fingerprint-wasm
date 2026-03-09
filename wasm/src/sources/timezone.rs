use wasm_bindgen::{JsCast, JsValue};

/**
 * Returns the timezone of the browser.
 * Uses Intl.DateTimeFormat().resolvedOptions().timeZone when available,
 * otherwise falls back to UTC offset.
 */
pub fn get_timezone(_ctx: &crate::sources::SourceContext) -> Result<JsValue, JsValue> {
    // Try to get timezone using Intl.DateTimeFormat
    if let Some(timezone) = try_get_timezone_from_intl() {
        return Ok(timezone);
    }

    // Fallback to UTC offset for browsers that don't support Intl
    let offset = -get_timezone_offset();
    let timezone_str = format!("UTC{}{}", if offset >= 0 { "+" } else { "" }, offset);
    Ok(JsValue::from_str(&timezone_str))
}

/// Try to get timezone using Intl.DateTimeFormat API
fn try_get_timezone_from_intl() -> Option<JsValue> {
    // Check if Intl is available
    let intl = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("Intl")).ok()?;
    if intl.is_undefined() || intl.is_null() {
        return None;
    }

    // Check if DateTimeFormat is available
    let date_time_format =
        js_sys::Reflect::get(&intl, &JsValue::from_str("DateTimeFormat")).ok()?;
    if date_time_format.is_undefined() || date_time_format.is_null() {
        return None;
    }

    // Create DateTimeFormat instance
    let dtf = js_sys::Reflect::construct(
        &date_time_format.dyn_into::<js_sys::Function>().ok()?,
        &js_sys::Array::new(),
    )
    .ok()?;

    // Check if resolvedOptions method exists
    let resolved_options =
        js_sys::Reflect::get(&dtf, &JsValue::from_str("resolvedOptions")).ok()?;
    if resolved_options.is_undefined() || resolved_options.is_null() {
        return None;
    }

    // Call resolvedOptions
    let options = resolved_options
        .dyn_into::<js_sys::Function>()
        .ok()?
        .call0(&dtf)
        .ok()?;

    // Get timeZone property
    let time_zone = js_sys::Reflect::get(&options, &JsValue::from_str("timeZone")).ok()?;
    if time_zone.is_undefined() || time_zone.is_null() {
        return None;
    }

    // Check if timezone is a non-empty string
    if let Some(timezone_str) = time_zone.as_string() {
        if !timezone_str.is_empty() {
            return Some(time_zone);
        }
    }

    None
}

fn get_timezone_offset() -> i32 {
    let current_year = js_sys::Date::new_0().get_full_year() as u32;

    // Get timezone offset for January and July (considering daylight saving time)
    let jan_offset =
        js_sys::Date::new_with_year_month_day(current_year, 0, 1).get_timezone_offset();
    let jul_offset =
        js_sys::Date::new_with_year_month_day(current_year, 6, 1).get_timezone_offset();

    // Return the larger offset (non-daylight saving time)
    jan_offset.max(jul_offset) as i32
}
