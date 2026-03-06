use crate::utils::browser::{is_chromium, is_gecko, is_webkit};
use wasm_bindgen::{JsCast, JsValue};

pub const STATUS_NO_GL_CONTEXT: i32 = -1;
pub const STATUS_GET_PARAMETER_NOT_A_FUNCTION: i32 = -2;

/**
 * WebGL fingerprinting.
 * Returns WebGL context information including version, vendor, renderer, and extensions.
 *
 * Some browsers block access to certain WebGL extensions for privacy reasons:
 * - Firefox blocks WEBGL_debug_renderer_info extension
 * - Chromium and WebKit block WEBGL_polygon_mode extension
 */
const VALID_CONTEXT_PARAMETERS: &[i32] = &[
    10752, 2849, 2884, 2885, 2886, 2928, 2929, 2930, 2931, 2932, 2960, 2961, 2962, 2963, 2964,
    2965, 2966, 2967, 2968, 2978, 3024, 3042, 3088, 3089, 3106, 3107, 32773, 32777, 32777, 32823,
    32824, 32936, 32937, 32938, 32939, 32968, 32969, 32970, 32971, 3317, 33170, 3333, 3379, 3386,
    33901, 33902, 34016, 34024, 34076, 3408, 3410, 3411, 3412, 3413, 3414, 3415, 34467, 34816,
    34817, 34818, 34819, 34877, 34921, 34930, 35660, 35661, 35724, 35738, 35739, 36003, 36004,
    36005, 36347, 36348, 36349, 37440, 37441, 37443, 7936, 7937, 7938,
];

const VALID_EXTENSION_PARAMS: &[i32] = &[
    34047, 35723, 36063, 34852, 34853, 34854, 34229, 36392, 36795, 38449,
];

const SHADER_TYPES: &[&str] = &["FRAGMENT_SHADER", "VERTEX_SHADER"];
const PRECISION_TYPES: &[&str] = &[
    "LOW_FLOAT",
    "MEDIUM_FLOAT",
    "HIGH_FLOAT",
    "LOW_INT",
    "MEDIUM_INT",
    "HIGH_INT",
];
const RENDERER_INFO_EXTENSION_NAME: &str = "WEBGL_debug_renderer_info";
const POLYGON_MODE_EXTENSION_NAME: &str = "WEBGL_polygon_mode";

pub fn get_web_gl_basics() -> Result<JsValue, JsValue> {
    let gl = get_webgl_context();

    if gl.is_null() {
        return Ok(JsValue::from(STATUS_NO_GL_CONTEXT));
    }

    if !is_valid_parameter_getter(&gl) {
        return Ok(JsValue::from(STATUS_GET_PARAMETER_NOT_A_FUNCTION));
    }

    let debug_extension = if should_avoid_debug_renderer_info() {
        JsValue::null()
    } else {
        get_extension(&gl, RENDERER_INFO_EXTENSION_NAME)
    };

    let result = js_sys::Object::new();

    let version = get_parameter(&gl, web_sys::WebGlRenderingContext::VERSION);
    let version_str = js_value_to_string(&version);
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("version"),
        &JsValue::from_str(&version_str),
    );

    let vendor = get_parameter(&gl, web_sys::WebGlRenderingContext::VENDOR);
    let vendor_str = js_value_to_string(&vendor);
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("vendor"),
        &JsValue::from_str(&vendor_str),
    );

    let vendor_unmasked = if !debug_extension.is_null() {
        let unmasked_vendor = get_extension_constant(&debug_extension, "UNMASKED_VENDOR_WEBGL");
        if !unmasked_vendor.is_null() {
            let value = get_parameter_by_code(&gl, &unmasked_vendor);
            js_value_to_string(&value)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("vendorUnmasked"),
        &JsValue::from_str(&vendor_unmasked),
    );

    let renderer = get_parameter(&gl, web_sys::WebGlRenderingContext::RENDERER);
    let renderer_str = js_value_to_string(&renderer);
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("renderer"),
        &JsValue::from_str(&renderer_str),
    );

    let renderer_unmasked = if !debug_extension.is_null() {
        let unmasked_renderer = get_extension_constant(&debug_extension, "UNMASKED_RENDERER_WEBGL");
        if !unmasked_renderer.is_null() {
            let value = get_parameter_by_code(&gl, &unmasked_renderer);
            js_value_to_string(&value)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("rendererUnmasked"),
        &JsValue::from_str(&renderer_unmasked),
    );

    let shading_language_version = get_parameter(
        &gl,
        web_sys::WebGlRenderingContext::SHADING_LANGUAGE_VERSION,
    );
    let shading_str = js_value_to_string(&shading_language_version);
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("shadingLanguageVersion"),
        &JsValue::from_str(&shading_str),
    );

    Ok(JsValue::from(result))
}

fn js_value_to_string(value: &JsValue) -> String {
    if value.is_null() || value.is_undefined() {
        return String::new();
    }
    if let Some(s) = value.as_string() {
        return s;
    }
    if let Ok(to_string) = js_sys::Reflect::get(value, &JsValue::from_str("toString")) {
        if let Ok(func) = to_string.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call0(value) {
                if let Some(s) = result.as_string() {
                    return s;
                }
            }
        }
    }
    String::new()
}

fn format_js_value(value: &JsValue) -> String {
    if value.is_null() {
        return "null".to_string();
    }
    if value.is_undefined() {
        return "undefined".to_string();
    }
    if let Some(s) = value.as_string() {
        return s;
    }
    if let Some(n) = value.as_f64() {
        if n.fract() == 0.0 && n >= i32::MIN as f64 && n <= u32::MAX as f64 {
            if n >= 0.0 && n > i32::MAX as f64 {
                return format!("{}", n as u32);
            }
            return format!("{}", n as i32);
        }
        return format!("{}", n);
    }
    if let Some(b) = value.as_bool() {
        return format!("{}", b);
    }
    if value.is_array() {
        let arr = js_sys::Array::from(value);
        let values: Vec<String> = arr.iter().map(|v| format_js_value(&v)).collect();
        return values.join(",");
    }
    if let Ok(to_string) = js_sys::Reflect::get(value, &JsValue::from_str("toString")) {
        if let Ok(func) = to_string.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call0(value) {
                if let Some(s) = result.as_string() {
                    return s;
                }
            }
        }
    }
    String::new()
}

pub fn get_web_gl_extensions() -> Result<JsValue, JsValue> {
    let gl = get_webgl_context();

    if gl.is_null() {
        return Ok(JsValue::from(STATUS_NO_GL_CONTEXT));
    }

    if !is_valid_parameter_getter(&gl) {
        return Ok(JsValue::from(STATUS_GET_PARAMETER_NOT_A_FUNCTION));
    }

    let extensions = get_supported_extensions(&gl);

    let result = js_sys::Object::new();

    let attributes_array = js_sys::Array::new();
    let context_attributes = get_context_attributes(&gl);
    if let Ok(attrs) = context_attributes {
        let keys = js_sys::Object::keys(&attrs);
        for i in 0..keys.length() {
            let key = keys.get(i);
            if let Ok(value) = js_sys::Reflect::get(&attrs, &key) {
                let key_str = key.as_string().unwrap_or_default();
                let value_str = if let Some(b) = value.as_bool() {
                    b.to_string()
                } else if let Some(s) = value.as_string() {
                    s
                } else {
                    format!("{:?}", value)
                };
                attributes_array.push(&JsValue::from_str(&format!("{}={}", key_str, value_str)));
            }
        }
    }
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("contextAttributes"),
        &attributes_array,
    );

    let parameters_array = js_sys::Array::new();
    let constants = get_constants_from_prototype(&gl);
    for constant in constants {
        let code = get_constant_value(&gl, &constant);
        if let Some(code_num) = code.as_f64() {
            let mut param_str = format!("{}={}", constant, code_num as i32);

            if VALID_CONTEXT_PARAMETERS.contains(&(code_num as i32)) {
                let value = get_parameter_by_code(&gl, &code);
                let value_str = format_js_value(&value);
                param_str = format!("{}={}", param_str, value_str);
            }

            parameters_array.push(&JsValue::from_str(&param_str));
        }
    }
    let mut params_vec: Vec<String> = parameters_array
        .iter()
        .filter_map(|v| v.as_string())
        .collect();
    params_vec.sort();
    let sorted_parameters = js_sys::Array::new();
    for p in params_vec {
        sorted_parameters.push(&JsValue::from_str(&p));
    }
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("parameters"),
        &sorted_parameters,
    );

    let shader_precisions_array = js_sys::Array::new();
    for shader_type in SHADER_TYPES {
        for precision_type in PRECISION_TYPES {
            let precision = get_shader_precision(&gl, shader_type, precision_type);
            shader_precisions_array.push(&JsValue::from_str(&format!(
                "{}.{}={}",
                shader_type,
                precision_type,
                precision.join(",")
            )));
        }
    }
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("shaderPrecisions"),
        &shader_precisions_array,
    );

    let extension_parameters_array = js_sys::Array::new();
    let unsupported_extensions_array = js_sys::Array::new();

    if let Some(ref exts) = extensions {
        for i in 0..exts.length() {
            let extension_name = exts.get(i);
            let extension_name_str = extension_name.as_string().unwrap_or_default();

            let should_skip_processing = (extension_name_str == RENDERER_INFO_EXTENSION_NAME
                && should_avoid_debug_renderer_info())
                || (extension_name_str == POLYGON_MODE_EXTENSION_NAME
                    && should_avoid_polygon_mode_extensions());

            if should_skip_processing {
                continue;
            }

            let extension = get_extension(&gl, &extension_name_str);
            if !extension.is_null() {
                let extension_constants = get_constants_from_prototype(&extension);

                for constant in extension_constants {
                    let code = get_constant_value(&extension, &constant);
                    if let Some(code_num) = code.as_f64() {
                        let mut param_str = format!("{}={}", constant, code_num as i32);

                        if VALID_EXTENSION_PARAMS.contains(&(code_num as i32)) {
                            let value = get_parameter_by_code(&gl, &code);
                            let value_str = format_js_value(&value);
                            param_str = format!("{}={}", param_str, value_str);
                        }

                        extension_parameters_array.push(&JsValue::from_str(&param_str));
                    }
                }
            } else {
                unsupported_extensions_array.push(&extension_name);
            }
        }
    }

    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("extensions"),
        &extensions
            .map(|e| JsValue::from(e))
            .unwrap_or(JsValue::null()),
    );

    let mut ext_params_vec: Vec<String> = extension_parameters_array
        .iter()
        .filter_map(|v| v.as_string())
        .collect();
    ext_params_vec.sort();
    let sorted_ext_params = js_sys::Array::new();
    for p in ext_params_vec {
        sorted_ext_params.push(&JsValue::from_str(&p));
    }
    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("extensionParameters"),
        &sorted_ext_params,
    );

    let _ = js_sys::Reflect::set(
        &result,
        &JsValue::from_str("unsupportedExtensions"),
        &unsupported_extensions_array,
    );

    Ok(JsValue::from(result))
}

fn get_webgl_context() -> JsValue {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(doc) => doc,
        None => return JsValue::null(),
    };

    let canvas = match document.create_element("canvas") {
        Ok(el) => match el.dyn_into::<web_sys::HtmlCanvasElement>() {
            Ok(c) => c,
            Err(_) => return JsValue::null(),
        },
        Err(_) => return JsValue::null(),
    };

    for context_type in &["webgl", "experimental-webgl"] {
        if let Ok(Some(ctx)) = canvas.get_context(context_type) {
            return ctx.into();
        }
    }

    JsValue::null()
}

fn get_parameter(gl: &JsValue, param: u32) -> JsValue {
    if let Ok(get_parameter) = js_sys::Reflect::get(gl, &JsValue::from_str("getParameter")) {
        if let Ok(func) = get_parameter.dyn_into::<js_sys::Function>() {
            return func
                .call1(gl, &JsValue::from(param))
                .unwrap_or(JsValue::null());
        }
    }
    JsValue::null()
}

fn get_parameter_by_code(gl: &JsValue, code: &JsValue) -> JsValue {
    if let Ok(get_parameter) = js_sys::Reflect::get(gl, &JsValue::from_str("getParameter")) {
        if let Ok(func) = get_parameter.dyn_into::<js_sys::Function>() {
            return func.call1(gl, code).unwrap_or(JsValue::null());
        }
    }
    JsValue::null()
}

fn get_extension(gl: &JsValue, name: &str) -> JsValue {
    if let Ok(get_extension) = js_sys::Reflect::get(gl, &JsValue::from_str("getExtension")) {
        if let Ok(func) = get_extension.dyn_into::<js_sys::Function>() {
            return func
                .call1(gl, &JsValue::from_str(name))
                .unwrap_or(JsValue::null());
        }
    }
    JsValue::null()
}

fn get_extension_constant(extension: &JsValue, name: &str) -> JsValue {
    js_sys::Reflect::get(extension, &JsValue::from_str(name)).unwrap_or(JsValue::null())
}

fn get_supported_extensions(gl: &JsValue) -> Option<js_sys::Array> {
    if let Ok(get_supported_extensions) =
        js_sys::Reflect::get(gl, &JsValue::from_str("getSupportedExtensions"))
    {
        if let Ok(func) = get_supported_extensions.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call0(gl) {
                if result.is_array() {
                    return Some(js_sys::Array::from(&result));
                }
            }
        }
    }
    None
}

fn get_context_attributes(gl: &JsValue) -> Result<js_sys::Object, JsValue> {
    if let Ok(get_context_attributes) =
        js_sys::Reflect::get(gl, &JsValue::from_str("getContextAttributes"))
    {
        if let Ok(func) = get_context_attributes.dyn_into::<js_sys::Function>() {
            return func.call0(gl).map(|v| js_sys::Object::from(v));
        }
    }
    Err(JsValue::null())
}

fn get_constants_from_prototype(obj: &JsValue) -> Vec<String> {
    let mut constants = Vec::new();

    if let Ok(prototype) = js_sys::Reflect::get(obj, &JsValue::from_str("__proto__")) {
        let keys = js_sys::Object::keys(&js_sys::Object::from(prototype));
        for i in 0..keys.length() {
            let key = keys.get(i);
            if let Some(key_str) = key.as_string() {
                if is_constant_like(&key_str) {
                    constants.push(key_str);
                }
            }
        }
    }

    constants
}

fn is_constant_like(key: &str) -> bool {
    key.chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' || c == 'x')
}

fn get_constant_value(obj: &JsValue, name: &str) -> JsValue {
    js_sys::Reflect::get(obj, &JsValue::from_str(name)).unwrap_or(JsValue::null())
}

fn get_shader_precision(gl: &JsValue, shader_type: &str, precision_type: &str) -> Vec<String> {
    let shader_type_code = get_constant_value(gl, shader_type);
    let precision_type_code = get_constant_value(gl, precision_type);

    if let Ok(get_shader_precision_format) =
        js_sys::Reflect::get(gl, &JsValue::from_str("getShaderPrecisionFormat"))
    {
        if let Ok(func) = get_shader_precision_format.dyn_into::<js_sys::Function>() {
            if let Ok(result) = func.call2(gl, &shader_type_code, &precision_type_code) {
                if !result.is_null() {
                    let range_min = js_sys::Reflect::get(&result, &JsValue::from_str("rangeMin"))
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as i32;
                    let range_max = js_sys::Reflect::get(&result, &JsValue::from_str("rangeMax"))
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as i32;
                    let precision = js_sys::Reflect::get(&result, &JsValue::from_str("precision"))
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as i32;
                    return vec![
                        range_min.to_string(),
                        range_max.to_string(),
                        precision.to_string(),
                    ];
                }
            }
        }
    }

    vec![]
}

fn should_avoid_debug_renderer_info() -> bool {
    is_gecko()
}

fn should_avoid_polygon_mode_extensions() -> bool {
    is_chromium() || is_webkit()
}

fn is_valid_parameter_getter(gl: &JsValue) -> bool {
    if let Ok(get_parameter) = js_sys::Reflect::get(gl, &JsValue::from_str("getParameter")) {
        if get_parameter.is_function() {
            return true;
        }
    }
    false
}
