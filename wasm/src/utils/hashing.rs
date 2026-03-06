use murmurhash3::murmurhash3_x64_128;

// 获取 UTF-8 字节，与 JavaScript 版本一致
fn get_utf8_bytes(input: &str) -> Vec<u8> {
    // 首先尝试快速路径：只包含 ASCII 字符
    let mut result = Vec::with_capacity(input.len());
    for c in input.chars() {
        let char_code = c as u32;
        if char_code > 127 {
            // 包含非 ASCII 字符，使用标准编码
            return input.as_bytes().to_vec();
        }
        result.push(char_code as u8);
    }
    result
}

// 计算 MurmurHash3 x64 128 位哈希，与 JavaScript 版本一致
pub fn x64hash128(input: &str) -> String {
    let key = get_utf8_bytes(input);
    let seed: u64 = 0;

    // 使用 murmurhash3 crate 计算哈希
    let (h1, h2) = murmurhash3_x64_128(&key, seed);

    // 与 JavaScript 版本相同的输出格式
    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        (h1 >> 32) as u32,
        (h1 & 0xffffffff) as u32,
        (h2 >> 32) as u32,
        (h2 & 0xffffffff) as u32
    )
}
