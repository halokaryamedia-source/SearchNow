use super::{
    CatalogError, CatalogItem, CatalogPage, CatalogProviderFailure, CatalogProviderItem,
    CatalogProviderPage, CatalogQuery, CatalogRequest,
};
use crate::{
    error::{BackendError, BackendResult},
    identity::valid_provider_key,
};
use std::{collections::HashMap, collections::HashSet, sync::Arc};

const MAX_QUERY_TEXT_BYTES: usize = 256;
const MAX_QUERY_TAGS: usize = 32;
const MAX_CONTENT_TYPE_FILTERS: usize = 16;
const MAX_PAGE_SIZE: u16 = 100;
const MAX_CURSOR_BYTES: usize = 512;
const MAX_ITEM_ID_BYTES: usize = 256;
const MAX_TITLE_BYTES: usize = 256;
const MAX_DESCRIPTION_BYTES: usize = 4 * 1024;
const MAX_ITEM_TAGS: usize = 32;
const MAX_TAG_BYTES: usize = 64;
const MAX_ITEM_TEXT_BYTES: usize = 8 * 1024;

pub trait CatalogProvider: Send + Sync {
    fn key(&self) -> &str;

    fn query(&self, query: &CatalogQuery) -> Result<CatalogProviderPage, CatalogProviderFailure>;
}

#[derive(Default)]
pub struct CatalogProviderRegistry {
    providers: HashMap<String, Arc<dyn CatalogProvider>>,
}

impl CatalogProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: Arc<dyn CatalogProvider>) -> BackendResult<()> {
        let key = provider.key().trim();
        if !valid_provider_key(key) {
            return Err(BackendError::new(
                "catalog_provider_key_invalid",
                "Catalog provider key is empty or unsupported.",
            ));
        }
        if self.providers.contains_key(key) {
            return Err(BackendError::new(
                "catalog_provider_duplicate",
                "A catalog provider with the same key is already registered.",
            ));
        }
        self.providers.insert(key.to_string(), provider);
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&Arc<dyn CatalogProvider>> {
        self.providers.get(key)
    }
}

pub struct CatalogService {
    providers: Arc<CatalogProviderRegistry>,
}

impl CatalogService {
    pub fn new(providers: Arc<CatalogProviderRegistry>) -> Self {
        Self { providers }
    }

    pub fn query(&self, request: &CatalogRequest) -> Result<CatalogPage, CatalogError> {
        validate_request(request)?;
        let provider = self.providers.get(&request.provider).ok_or_else(|| {
            CatalogError::new(
                "catalog_provider_unavailable",
                "The requested catalog provider is not available in this SearchNow build.",
                false,
            )
        })?;
        let raw_page = provider
            .query(&request.query)
            .map_err(map_provider_failure)?;
        normalize_page(&request.provider, &request.query, raw_page)
    }
}

fn validate_request(request: &CatalogRequest) -> Result<(), CatalogError> {
    if !valid_provider_key(&request.provider) {
        return Err(invalid_request("Catalog provider identity is invalid."));
    }
    if let Some(text) = &request.query.text {
        if text.is_empty()
            || text.len() > MAX_QUERY_TEXT_BYTES
            || text.chars().any(char::is_control)
        {
            return Err(invalid_request(
                "Catalog search text is invalid or too long.",
            ));
        }
    }
    if request.query.filters.content_types.len() > MAX_CONTENT_TYPE_FILTERS {
        return Err(invalid_request(
            "Catalog content-type filters exceed the supported bound.",
        ));
    }
    if request.query.filters.tags.len() > MAX_QUERY_TAGS
        || request
            .query
            .filters
            .tags
            .iter()
            .any(|tag| !valid_compact_text(tag, MAX_TAG_BYTES))
    {
        return Err(invalid_request(
            "Catalog tag filters are invalid or too numerous.",
        ));
    }
    if request.query.page.limit == 0 || request.query.page.limit > MAX_PAGE_SIZE {
        return Err(invalid_request(
            "Catalog page size is outside the supported bound.",
        ));
    }
    if request
        .query
        .page
        .cursor
        .as_ref()
        .is_some_and(|cursor| !valid_compact_text(cursor, MAX_CURSOR_BYTES))
    {
        return Err(invalid_request(
            "Catalog page cursor is invalid or too long.",
        ));
    }
    Ok(())
}

fn normalize_page(
    provider: &str,
    query: &CatalogQuery,
    page: CatalogProviderPage,
) -> Result<CatalogPage, CatalogError> {
    if page.items.len() > query.page.limit as usize || page.items.len() > MAX_PAGE_SIZE as usize {
        return Err(invalid_provider_data());
    }
    if page
        .next_cursor
        .as_ref()
        .is_some_and(|cursor| !valid_compact_text(cursor, MAX_CURSOR_BYTES))
    {
        return Err(invalid_provider_data());
    }

    let mut seen_ids = HashSet::with_capacity(page.items.len());
    let mut items = Vec::with_capacity(page.items.len());
    for item in page.items {
        validate_provider_item(&item)?;
        if !seen_ids.insert(item.item_id.clone()) {
            return Err(invalid_provider_data());
        }
        items.push(CatalogItem {
            provider: provider.to_string(),
            item_id: item.item_id,
            title: item.title,
            description: item.description,
            content_type: item.content_type,
            tags: item.tags,
            published_at_ms: item.published_at_ms,
            updated_at_ms: item.updated_at_ms,
            download: item.download,
        });
    }

    Ok(CatalogPage {
        provider: provider.to_string(),
        items,
        next_cursor: page.next_cursor,
    })
}

fn validate_provider_item(item: &CatalogProviderItem) -> Result<(), CatalogError> {
    if !valid_compact_text(&item.item_id, MAX_ITEM_ID_BYTES)
        || item.item_id.contains("://")
        || !valid_compact_text(&item.title, MAX_TITLE_BYTES)
        || item.tags.len() > MAX_ITEM_TAGS
        || item
            .tags
            .iter()
            .any(|tag| !valid_compact_text(tag, MAX_TAG_BYTES))
    {
        return Err(invalid_provider_data());
    }

    if item
        .description
        .as_ref()
        .is_some_and(|value| !valid_description(value))
    {
        return Err(invalid_provider_data());
    }

    let descriptive_bytes = item.title.len()
        + item.description.as_ref().map_or(0, String::len)
        + item.tags.iter().map(String::len).sum::<usize>();
    if descriptive_bytes > MAX_ITEM_TEXT_BYTES {
        return Err(invalid_provider_data());
    }

    if let Some(download) = &item.download {
        download
            .to_download_source()
            .map_err(|_| invalid_provider_data())?;
    }
    Ok(())
}

fn valid_compact_text(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes && !value.chars().any(char::is_control)
}

fn valid_description(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_DESCRIPTION_BYTES
        && !value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
}

fn map_provider_failure(failure: CatalogProviderFailure) -> CatalogError {
    let code = if valid_provider_key(&failure.code) {
        failure.code
    } else {
        "catalog_provider_query_failed".to_string()
    };
    CatalogError::new(
        code,
        "Catalog provider could not complete the requested query.",
        failure.retryable,
    )
}

fn invalid_request(message: &'static str) -> CatalogError {
    CatalogError::new("catalog_query_invalid", message, false)
}

fn invalid_provider_data() -> CatalogError {
    CatalogError::new(
        "catalog_provider_data_invalid",
        "Catalog provider returned malformed or unsupported item data.",
        false,
    )
}
