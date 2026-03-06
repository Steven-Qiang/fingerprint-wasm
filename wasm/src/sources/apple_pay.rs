use crate::utils::dom::is_any_parent_cross_origin;
use wasm_bindgen::{JsCast, JsValue, throw_val};

// Apple Pay state enum
enum ApplePayState {
    Disabled = 0, // Apple Pay is disabled on the user device
    Enabled = 1,  // Apple Pay is enabled on the user device
    NoAPI = -1,   // The browser doesn't have the API to work with Apple Pay
    NotAvailableInInsecureContext = -2, /* Using Apple Pay isn't allowed because the page
                   * context isn't secure (not HTTPS) */
    NotAvailableInFrame = -3, /* Using Apple Pay isn't allowed because the code runs in a frame
                               * with cross-origin parent */
}

// Returns the Apple Pay state
pub async fn get_apple_pay_state() -> Result<JsValue, JsValue> {
    let result = inner_get_apple_pay_state();
    Ok(JsValue::from(result))
}

// Internal implementation
fn inner_get_apple_pay_state() -> i32 {
    if let Some(window) = web_sys::window() {
        if let Ok(apple_pay_session) =
            js_sys::Reflect::get(&window, &JsValue::from_str("ApplePaySession"))
        {
            if let Ok(can_make_payments) =
                js_sys::Reflect::get(&apple_pay_session, &JsValue::from_str("canMakePayments"))
            {
                if can_make_payments.is_function() {
                    // Check if running in a cross-origin frame
                    if will_print_console_error() {
                        return ApplePayState::NotAvailableInFrame as i32;
                    }

                    // Try to call canMakePayments()
                    match can_make_payments.dyn_into::<js_sys::Function>() {
                        Ok(func) => match func.call0(&apple_pay_session) {
                            Ok(result) => {
                                if let Some(bool_value) = result.as_bool() {
                                    return if bool_value {
                                        ApplePayState::Enabled as i32
                                    } else {
                                        ApplePayState::Disabled as i32
                                    };
                                }
                            }
                            Err(error) => {
                                return get_state_from_error(error);
                            }
                        },
                        Err(_) => {
                            // If canMakePayments is not a function, return NoAPI
                            return ApplePayState::NoAPI as i32;
                        }
                    }
                }
            }
        }
    }

    ApplePayState::NoAPI as i32
}

// Checks if the code runs in a cross-origin frame
fn will_print_console_error() -> bool {
    is_any_parent_cross_origin()
}

// Gets the Apple Pay state from an error
fn get_state_from_error(error: JsValue) -> i32 {
    if let Some(error) = error.dyn_ref::<js_sys::Error>() {
        if error.name() == "InvalidAccessError" {
            let message = error.message();
            if let Some(message_str) = message.as_string() {
                if message_str.to_lowercase().contains("from")
                    && message_str.to_lowercase().contains("insecure")
                {
                    return ApplePayState::NotAvailableInInsecureContext as i32;
                }
            }
        }
    }

    // Re-throw the error if it's not the expected one
    throw_val(error);
}
