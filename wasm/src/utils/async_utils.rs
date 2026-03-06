use wasm_bindgen::{
    JsCast,
    prelude::{Closure, JsValue},
};
use wasm_bindgen_futures::JsFuture;

// Waits for a specified number of milliseconds
pub async fn wait(milliseconds: u32) -> Result<(), JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let closure = Closure::wrap(Box::new(move || {
            resolve
                .call1(&JsValue::undefined(), &JsValue::undefined())
                .unwrap();
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                milliseconds.try_into().unwrap(),
            )
            .unwrap();
        closure.forget();
    });

    JsFuture::from(promise).await?;
    Ok(())
}
