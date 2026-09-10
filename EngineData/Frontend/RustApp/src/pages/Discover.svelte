<script lang="ts">
  import { Search } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import { catalogContentTypeLabel } from "../app/shared/format";
  import type {
    CatalogContentType,
    CatalogPage,
    CatalogRequest,
    CatalogSort,
    ProviderRuntimeStatus,
  } from "../app/shared/types";
  import Notice from "../components/ui/Notice.svelte";
  import PageState from "../components/ui/PageState.svelte";

  type ContentFilter = "all" | CatalogContentType;

  let {
    runtimeReady,
    providers,
    active,
  }: {
    runtimeReady: boolean;
    providers: ProviderRuntimeStatus[];
    active: boolean;
  } = $props();

  let query = $state("");
  let contentFilter = $state<ContentFilter>("all");
  let sort = $state<CatalogSort>("relevance");
  let selectedProvider = $state("");
  let page = $state<CatalogPage | null>(null);
  let loading = $state(false);
  let loadingMore = $state(false);
  let error = $state("");
  let requestSequence = 0;

  let catalogProviders = $derived(providers.filter((provider) => provider.capabilities.catalog));

  $effect(() => {
    const firstProvider = catalogProviders[0]?.capabilities.provider ?? "";
    const stillAvailable = catalogProviders.some(
      (provider) => provider.capabilities.provider === selectedProvider,
    );
    if (!stillAvailable && selectedProvider !== firstProvider) selectedProvider = firstProvider;
  });

  function makeRequest(
    provider: string,
    text: string,
    filter: ContentFilter,
    selectedSort: CatalogSort,
    cursor: string | null,
  ): CatalogRequest {
    return {
      provider,
      query: {
        text: text.trim() || null,
        filters: {
          contentTypes: filter === "all" ? [] : [filter],
          tags: [],
        },
        sort: selectedSort,
        page: { limit: 30, cursor },
      },
    };
  }

  async function queryCatalog(
    provider: string,
    text: string,
    filter: ContentFilter,
    selectedSort: CatalogSort,
    cursor: string | null = null,
    append = false,
  ): Promise<void> {
    if (!runtimeReady || !active || !provider) return;
    const sequence = ++requestSequence;
    if (append) loadingMore = true;
    else loading = true;
    error = "";

    const result = await runtimeProductFacade.queryCatalog(
      makeRequest(provider, text, filter, selectedSort, cursor),
    );
    if (sequence !== requestSequence) return;

    if (result.ok) {
      if (append && page) {
        page = {
          ...result.data,
          items: [...page.items, ...result.data.items],
        };
      } else {
        page = result.data;
      }
    } else {
      error = result.error.message;
      if (!append) page = null;
    }
    loading = false;
    loadingMore = false;
  }

  async function loadMore(): Promise<void> {
    if (!page?.nextCursor || loadingMore) return;
    await queryCatalog(selectedProvider, query, contentFilter, sort, page.nextCursor, true);
  }

  $effect(() => {
    const provider = selectedProvider;
    const text = query;
    const filter = contentFilter;
    const selectedSort = sort;
    if (!active || !runtimeReady || !provider) return;
    const timer = setTimeout(() => {
      void queryCatalog(provider, text, filter, selectedSort);
    }, 320);
    return () => clearTimeout(timer);
  });
</script>

<section class="page" hidden={!active}>
  <div class="page-heading">
    <div>
      <span class="eyebrow">Catalog</span>
      <h1>Discover</h1>
      <p>Search connected catalog sources for worlds, Add-Ons, packs, skins, and persona content.</p>
    </div>
  </div>

  {#if catalogProviders.length > 0}
    <div class="toolbar toolbar--catalog">
      <label class="search-field search-field--wide">
        <Search size={15} aria-hidden="true" />
        <input bind:value={query} type="search" placeholder="Search catalog" aria-label="Search catalog" />
      </label>
      {#if catalogProviders.length > 1}
        <select class="select-field" bind:value={selectedProvider} aria-label="Catalog provider">
          {#each catalogProviders as provider (provider.capabilities.provider)}
            <option value={provider.capabilities.provider}>{provider.capabilities.provider}</option>
          {/each}
        </select>
      {/if}
      <select class="select-field" bind:value={contentFilter} aria-label="Catalog content type">
        <option value="all">All types</option>
        <option value="world">Worlds</option>
        <option value="addon">Add-Ons</option>
        <option value="resourcePack">Resource Packs</option>
        <option value="skin">Skins</option>
        <option value="persona">Persona</option>
      </select>
      <select class="select-field" bind:value={sort} aria-label="Catalog sort">
        <option value="relevance">Relevance</option>
        <option value="newest">Newest</option>
        <option value="oldest">Oldest</option>
        <option value="nameAsc">Name A–Z</option>
        <option value="nameDesc">Name Z–A</option>
      </select>
    </div>
  {:else}
    <div class="search-shell" aria-disabled="true">
      <span><Search size={14} aria-hidden="true" /> Search catalog</span>
      <kbd>Source unavailable</kbd>
    </div>
  {/if}

  {#if error}
    <Notice tone="warning" title="Catalog unavailable." message={error} />
  {/if}

  {#if !runtimeReady}
    <PageState marker="02" title="Discover unavailable" message="SearchNow could not connect to the desktop runtime needed to browse catalog sources." />
  {:else if catalogProviders.length === 0}
    <PageState marker="02" title="No catalog source connected" message="You can continue using your local Library and Downloads. Catalog browsing will become available when a source is connected." />
  {:else if loading && !page}
    <PageState kind="loading" title="Searching catalog" message="Waiting for results from the selected catalog source." />
  {:else if page && page.items.length > 0}
    <div class="content-grid content-grid--catalog" aria-busy={loading}>
      {#each page.items as item (`${item.provider}:${item.itemId}`)}
        <article class="content-card content-card--catalog">
          <div class="content-card__preview" aria-hidden="true"><span>{item.contentType === "world" ? "W" : item.contentType === "addon" ? "A" : "C"}</span></div>
          <div class="content-card__body">
            <div class="content-card__meta"><span>{catalogContentTypeLabel(item.contentType)}</span><span class="chip">{item.provider}</span></div>
            <h2 title={item.title}>{item.title}</h2>
            <p>{item.description ?? "No catalog description is available for this item."}</p>
            {#if item.tags.length}
              <div class="chip-row">{#each item.tags.slice(0, 4) as tag}<span class="chip">{tag}</span>{/each}</div>
            {/if}
            <div class="content-card__footer">
              <span class="state-text">{item.download ? "Download available" : "Browse only"}</span>
            </div>
          </div>
        </article>
      {/each}
    </div>
    {#if page.nextCursor}
      <div class="center-actions">
        <button class="button button--secondary" type="button" onclick={loadMore} disabled={loadingMore}>{loadingMore ? "Loading" : "Load more"}</button>
      </div>
    {/if}
  {:else if page}
    <PageState marker="02" title="No matching catalog content" message="Try a broader search or another content type." />
  {/if}
</section>
