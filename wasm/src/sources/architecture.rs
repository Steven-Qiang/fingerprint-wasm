use wasm_bindgen::JsValue;

// Returns the architecture identifier
// Unlike most other architectures, on x86/x86-64 when floating-point instructions
// have no NaN arguments, but produce NaN output, the output NaN has sign bit set.
// We use it to distinguish x86/x86-64 from other architectures, by doing subtraction
// of two infinities (must produce NaN per IEEE 754 standard).
pub fn get_architecture() -> Result<JsValue, JsValue> {
    let result = inner_get_architecture();
    Ok(JsValue::from(result))
}

// Internal implementation
fn inner_get_architecture() -> u8 {
    // Create a Float32Array with one element
    let f = js_sys::Float32Array::new_with_length(1);

    // Create a Uint8Array view of the same buffer
    let u8 = js_sys::Uint8Array::new_with_byte_offset_and_length(&f.buffer(), 0, 4);

    // Set the first element to Infinity
    f.set_index(0, f32::INFINITY);

    // Subtract Infinity from Infinity, which should produce NaN
    f.set_index(0, f.get_index(0) - f.get_index(0));

    // Return the 4th byte (sign bit for x86/x86-64)
    u8.get_index(3)
}
