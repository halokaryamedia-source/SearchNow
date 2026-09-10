<script lang="ts">
  import { onMount } from "svelte";
  import { AlertTriangle, RefreshCw, Search } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import { localContentTypeLabel } from "../app/shared/format";
  import type { LocalBackendSnapshot, LocalContentItem, LocalContentType } from "../app/shared/types";

  type LibraryFilter = "all" | LocalContentType | "issues";

  let { runtimeReady }: { runtimeReady: boolean } = $props();
  let loading = $state(false);
  let loaded = $state(false);
  let snapshot = $state<LocalBackendSnapshot | null>(null);
  let error = $state("");
  let query = $state("");
  let filter = $state<LibraryFilter>("all");

  let filteredItems = $derived(
    (snapshot?.library.items ?? []).filter((item) => matchesCurrentFilter(item)),
  );

  function matchesCurrentFilter(item: LocalContentItem): boolean {
    if (filter === "issues" && item.status !== "invalidMetadata") return false;
    if (filter !== "all" && filter !== "issues" && item.contentType !== filter) return false;
    const needle = query.trim().toLowerCase();
    if (!needle) return true;
    return `${item.title} ${item.description ?? ""}`.toLowerCase().includes(needle);
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

  onMount(() => {
    if (runtimeReady) void refresh();
    else loaded = true;
  });
</script>

<section class="page">
  <div class="page-heading page-heading--actions">
    <div>
      <span class="eyebrow">My content</span>
      <h1>Library</h1>
      <p>Installed Minecraft Bedrock worlds and packs detected from the runtime-owned local library.</p>
    </div>
    <button class="button button--secondary" type="button" onclick={refresh} disabled={!runtimeReady || loading}>
      <RefreshCw size={15} class:spin={loading} />
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
    </div>
  {/if}

  {#if snapshot?.library.warnings.length}
    <article class="notice notice--warning">
      <AlertTriangle size={16} />
      <div>
        <strong>Library scan completed with {snapshot.library.warnings.length} warning{snapshot.library.warnings.length === 1 ? "" : "s"}.</strong>
        <span>{snapshot.library.warnings[0].message}</span>
      </div>
    </article>
  {/if}

  {#if !runtimeReady}
    <article class="empty-panel">
      <div class="empty-panel__icon">01</div>
      <div>
        <h2>Desktop runtime unavailable</h2>
        <p>The Library UI is ready, but reading Minecraft content requires the Tauri runtime.</p>
      </div>
    </article>
  {:else if loading && !loaded}
    <article class="empty-panel">
      <div class="empty-panel__icon"><RefreshCw size={18} class="spin" /></div>
      <div><h2>Scanning local content</h2><p>SearchNow is reading the bounded Minecraft locations owned by the Rust backend.</p></div>
    </article>
  {:else if error}
    <article class="empty-panel empty-panel--error">
      <div class="empty-panel__icon"><AlertTriangle size={18} /></div>
      <div>
        <h2>Library scan unavailable</h2>
        <p>{error}</p>
        <button class="button button--secondary button--compact" type="button" onclick={refresh}>Try again</button>
      </div>
    </article>
  {:else if snapshot && filteredItems.length > 0}
    <div class="content-grid">
      {#each filteredItems as item (item.id)}
        <article class="content-card">
          <div class="content-card__preview">
            <span>{item.contentType === "world" ? "W" : "P"}</span>
          </div>
          <div class="content-card__body">
            <div class="content-card__meta">
              <span>{localContentTypeLabel(item.contentType)}</span>
              {#if item.isDevelopment}<span class="chip">Development</span>{/if}
            </div>
            <h2>{item.title}</h2>
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
    <article class="empty-panel">
      <div class="empty-panel__icon">01</div>
      <div>
        <h2>{snapshot.library.items.length ? "No matching content" : "No local content found"}</h2>
        <p>{snapshot.library.items.length ? "Change the search or content filter to see other items." : snapshot.minecraft.message}</p>
      </div>
    </article>
  {/if}
</section>
