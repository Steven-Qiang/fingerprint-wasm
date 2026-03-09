use murmurhash3::murmurhash3_x64_128;

// Get UTF-8 bytes, consistent with JavaScript version
fn get_utf8_bytes(input: &str) -> Vec<u8> {
    // First try fast path: only contains ASCII characters
    let mut result = Vec::with_capacity(input.len());
    for c in input.chars() {
        let char_code = c as u32;
        if char_code > 127 {
            // Contains non-ASCII characters, use standard encoding
            return input.as_bytes().to_vec();
        }
        result.push(char_code as u8);
    }
    result
}

// Calculate MurmurHash3 x64 128-bit hash, consistent with JavaScript version
pub fn x64hash128(input: &str) -> String {
    let key = get_utf8_bytes(input);
    let seed: u64 = 0;

    // Use murmurhash3 crate to calculate hash
    let (h1, h2) = murmurhash3_x64_128(&key, seed);

    // Same output format as JavaScript version
    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        (h1 >> 32) as u32,
        (h1 & 0xffffffff) as u32,
        (h2 >> 32) as u32,
        (h2 & 0xffffffff) as u32
    )
}
