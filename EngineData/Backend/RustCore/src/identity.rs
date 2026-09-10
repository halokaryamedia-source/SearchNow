pub const MAX_PROVIDER_KEY_BYTES: usize = 64;
pub const MAX_STABLE_RESOURCE_ID_BYTES: usize = 2_048;
pub const MAX_DOWNLOAD_RESOURCE_ID_BYTES: usize = 4_096;

pub fn valid_provider_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_PROVIDER_KEY_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub fn valid_stable_resource_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_STABLE_RESOURCE_ID_BYTES
        && !value.chars().any(char::is_control)
        && !value.contains("://")
        && !value.contains('?')
        && !value.contains('#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_key_contract_is_compact_and_stable() {
        assert!(valid_provider_key("minecraft-marketplace"));
        assert!(!valid_provider_key("minecraft marketplace"));
        assert!(!valid_provider_key(""));
    }

    #[test]
    fn stable_resource_identity_rejects_runtime_url_material() {
        assert!(valid_stable_resource_id("catalog:item:123"));
        assert!(!valid_stable_resource_id("https://example.test/item"));
        assert!(!valid_stable_resource_id("item?token=secret"));
        assert!(!valid_stable_resource_id("item#fragment"));
    }
}
