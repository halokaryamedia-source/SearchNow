pub(crate) const MAX_PROVIDER_KEY_BYTES: usize = 64;
pub(crate) const MAX_STABLE_RESOURCE_ID_BYTES: usize = 512;

pub(crate) fn valid_provider_key(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= MAX_PROVIDER_KEY_BYTES
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub(crate) fn valid_stable_resource_id(value: &str) -> bool {
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
    fn provider_keys_are_compact_and_transport_safe() {
        assert!(valid_provider_key("marketplace.v1"));
        assert!(!valid_provider_key("marketplace provider"));
        assert!(!valid_provider_key(""));
    }

    #[test]
    fn stable_resource_ids_reject_runtime_url_material() {
        assert!(valid_stable_resource_id("asset/123"));
        assert!(!valid_stable_resource_id("https://example.invalid/asset"));
        assert!(!valid_stable_resource_id("asset?token=secret"));
        assert!(!valid_stable_resource_id("asset#fragment"));
    }
}
