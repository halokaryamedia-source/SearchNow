<script lang="ts">
  import { RefreshCw, Search } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import { localContentTypeLabel } from "../app/shared/format";
  import type { LocalBackendSnapshot, LocalContentItem, LocalContentType } from "../app/shared/types";
  import Notice from "../components/ui/Notice.svelte";
  import PageState from "../components/ui/PageState.svelte";

  type LibraryFilter = "all" | LocalContentType | "issues";
  type LibrarySort = "nameAsc" | "nameDesc" | "type" | "status";

  let { runtimeReady, active }: { runtimeReady: boolean; active: boolean } = $props();
  let loading = $state(false);
  let loaded = $state(false);
  let snapshot = $state<LocalBackendSnapshot | null>(null);
  let error = $state("");
  let query = $state("");
  let filter = $state<LibraryFilter>("all");
  let sort = $state<LibrarySort>("nameAsc");

  let filteredItems = $derived(
    (snapshot?.library.items ?? [])
      .filter((item) => matchesCurrentFilter(item))
      .slice()
      .sort(compareItems),
  );

  function matchesCurrentFilter(item: LocalContentItem): boolean {
    if (filter === "issues" && item.status !== "invalidMetadata") return false;
    if (filter !== "all" && filter !== "issues" && item.contentType !== filter) return false;
    const needle = query.trim().toLowerCase();
    if (!needle) return true;
    return `${item.title} ${item.description ?? ""}`.toLowerCase().includes(needle);
  }

  function compareItems(left: LocalContentItem, right: LocalContentItem): number {
    if (sort === "nameDesc") return right.title.localeCompare(left.title);
    if (sort === "type") {
      return localContentTypeLabel(left.contentType).localeCompare(localContentTypeLabel(right.contentType)) || left.title.localeCompare(right.title);
    }
    if (sort === "status") {
      return left.status.localeCompare(right.status) || left.title.localeCompare(right.title);
    }
    return left.title.localeCompare(right.title);
  }

  async function refresh(): Promise<void> {
    if (!runtimeReady || loading) return;
    loading = true;
    error = "";
    const result = await runtimeProductFacade.loadLibrary();
    if (result.ok) snapshot = result.data;
    else error = result.error.message;
    loaded = true;
    loading = false;
  }

  $effect(() => {
    if (!active || !runtimeReady) return;
    if (!loaded && !loading) void refresh();
  });
</script>

<section class="page" hidden={!active}>
  <div class="page-heading page-heading--actions">
    <div>
      <span class="eyebrow">My content</span>
      <h1>Library</h1>
      <p>Browse Minecraft Bedrock worlds and packs detected on this device.</p>
    </div>
    <button class="button button--secondary" type="button" onclick={refresh} disabled={!runtimeReady || loading}>
      <RefreshCw size={15} class={loading ? "spin" : ""} />
      {loading ? "Scanning" : "Rescan"}
    </button>
  </div>

  <div class="metric-grid metric-grid--four">
    <article class="metric-card">
      <span>Total content</span>
      <strong>{snapshot?.library.summary.total ?? "—"}</strong>
      <small>{snapshot ? `${snapshot.library.scannedRoots} storage root${snapshot.library.scannedRoots === 1 ? "" : "s"} scanned` : "Awaiting scan"}</small>
    </article>
    <article class="metric-card">
      <span>Worlds</span>
      <strong>{snapshot?.library.summary.worlds ?? "—"}</strong>
      <small>Local Minecraft worlds</small>
    </article>
    <article class="metric-card">
      <span>Packs</span>
      <strong>{snapshot ? snapshot.library.summary.behaviorPacks + snapshot.library.summary.resourcePacks + snapshot.library.summary.skinPacks : "—"}</strong>
      <small>Behavior, resource, and skin packs</small>
    </article>
    <article class="metric-card">
      <span>Needs review</span>
      <strong>{snapshot?.library.summary.invalidItems ?? "—"}</strong>
      <small>Items with invalid metadata</small>
    </article>
  </div>

  {#if snapshot}
    <div class="toolbar">
      <label class="search-field">
        <Search size={15} />
        <input bind:value={query} type="search" placeholder="Search your library" aria-label="Search local library" />
      </label>
      <select class="select-field" bind:value={filter} aria-label="Filter content type">
        <option value="all">All content</option>
        <option value="world">Worlds</option>
        <option value="behaviorPack">Behavior packs</option>
        <option value="resourcePack">Resource packs</option>
        <option value="skinPack">Skin packs</option>
        <option value="issues">Needs review</option>
      </select>
      <select class="select-field" bind:value={sort} aria-label="Sort local library">
        <option value="nameAsc">Name A–Z</option>
        <option value="nameDesc">Name Z–A</option>
        <option value="type">Content type</option>
        <option value="status">Review status</option>
      </select>
    </div>
  {/if}

  {#if snapshot?.library.warnings.length}
    <Notice
      tone="warning"
      title={`Library scan completed with ${snapshot.library.warnings.length} warning${snapshot.library.warnings.length === 1 ? "" : "s"}.`}
      message={snapshot.library.warnings[0].message}
    />
  {/if}

  {#if !runtimeReady}
    <PageState marker="01" title="Local library unavailable" message="SearchNow could not connect to the desktop runtime needed to read content from this device." />
  {:else if loading && !loaded}
    <PageState kind="loading" title="Scanning local content" message="Checking the Minecraft locations configured for this device." />
  {:else if error}
    <PageState kind="error" title="Library scan unavailable" message={error} actionLabel="Try again" onAction={refresh} />
  {:else if snapshot && filteredItems.length > 0}
    <div class="content-grid">
      {#each filteredItems as item (item.id)}
        <article class="content-card">
          <div class="content-card__preview" aria-hidden="true">
            <span>{item.contentType === "world" ? "W" : "P"}</span>
          </div>
          <div class="content-card__body">
            <div class="content-card__meta">
              <span>{localContentTypeLabel(item.contentType)}</span>
              {#if item.isDevelopment}<span class="chip">Development</span>{/if}
            </div>
            <h2 title={item.title}>{item.title}</h2>
            <p>{item.description ?? "No description is available for this local item."}</p>
            <div class="content-card__footer">
              <span class:state-text--warning={item.status === "invalidMetadata"} class="state-text">
                {item.status === "ready" ? "Ready" : "Needs review"}
              </span>
              {#if item.version.length}<span>v{item.version.join(".")}</span>{/if}
            </div>
            <details class="technical-details">
              <summary>Technical details</summary>
              <dl>
                <div><dt>Location</dt><dd>{item.path}</dd></div>
                <div><dt>Storage root</dt><dd>{item.rootId}</dd></div>
                {#if item.manifestUuid}<div><dt>Manifest UUID</dt><dd>{item.manifestUuid}</dd></div>{/if}
                {#if item.issue}<div><dt>Issue</dt><dd>{item.issue}</dd></div>{/if}
              </dl>
            </details>
          </div>
        </article>
      {/each}
    </div>
  {:else if snapshot}
    <PageState
      marker="01"
      title={snapshot.library.items.length ? "No matching content" : "No local content found"}
      message={snapshot.library.items.length ? "Change the search, filter, or sort controls to see other items." : snapshot.minecraft.message}
    />
  {/if}
</section>
