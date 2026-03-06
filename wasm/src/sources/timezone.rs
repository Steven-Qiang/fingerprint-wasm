use wasm_bindgen::JsValue;

/**
 * Returns the timezone of the browser.
 * Uses Intl.DateTimeFormat().resolvedOptions().timeZone when available,
 * otherwise falls back to UTC offset.
 */
pub fn get_timezone() -> Result<JsValue, JsValue> {
    let result = js_sys::eval(
        r#"
        (function() {
            if (typeof Intl !== 'undefined' && Intl.DateTimeFormat) {
                const dtf = new Intl.DateTimeFormat();
                if (dtf.resolvedOptions && dtf.resolvedOptions().timeZone) {
                    return dtf.resolvedOptions().timeZone;
                }
            }
            return null;
        })()
    "#,
    )?;

    if let Some(timezone) = result.as_string() {
        if !timezone.is_empty() {
            return Ok(JsValue::from_str(&timezone));
        }
    }

    // 对于不支持时区名称的浏览器，使用偏移量
    let offset = -get_timezone_offset();
    let timezone_str = format!("UTC{}{}", if offset >= 0 { "+" } else { "" }, offset);
    Ok(JsValue::from_str(&timezone_str))
}

fn get_timezone_offset() -> i32 {
    let current_year = js_sys::Date::new_0().get_full_year() as u32;

    // 获取 1 月和 7 月的时区偏移（考虑夏令时）
    let jan_offset =
        js_sys::Date::new_with_year_month_day(current_year, 0, 1).get_timezone_offset();
    let jul_offset =
        js_sys::Date::new_with_year_month_day(current_year, 6, 1).get_timezone_offset();

    // 返回较大的偏移（非夏令时）
    jan_offset.max(jul_offset) as i32
}
