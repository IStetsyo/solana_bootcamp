pub fn parse_string_key(key: &str) -> String {
    let parsed_key = if key.starts_with('[') {
        key[1..key.len() - 1].to_string()
    } else {
        return key.to_string();
    };
    return parsed_key;
}

pub fn parse_u8_key(key: &str) -> Vec<u8> {
    let secret_key_bytes: Vec<u8> = key
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u8>()
                .expect("Failed to parse secret key part")
        })
        .collect();
    return secret_key_bytes;
}
