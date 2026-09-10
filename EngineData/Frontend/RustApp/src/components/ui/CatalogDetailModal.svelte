<script lang="ts">
  import { X } from "@lucide/svelte";
  import { catalogContentTypeLabel, formatBytes, formatDate } from "../../app/shared/format";
  import type { CatalogItem } from "../../app/shared/types";
  import ContentTypeMark from "./ContentTypeMark.svelte";

  let {
    item,
    open,
    onClose,
    onDownload = null,
    downloadBusy = false,
  }: {
    item: CatalogItem | null;
    open: boolean;
    onClose: () => void;
    onDownload?: (() => void) | null;
    downloadBusy?: boolean;
  } = $props();

  let canDownload = $derived(Boolean(item?.download && item?.fileName && onDownload));

  function handleBackdrop(event: MouseEvent): void {
    if (event.target === event.currentTarget) onClose();
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (open && event.key === "Escape") onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && item}
  <div class="catalog-modal__backdrop" role="presentation" onclick={handleBackdrop}>
    <section class="catalog-modal" role="dialog" aria-modal="true" aria-labelledby="catalog-modal-title">
      <button class="catalog-modal__close" type="button" aria-label="Close details" onclick={onClose}>
        <X size={18} aria-hidden="true" />
      </button>

      <div class="catalog-modal__media">
        {#if item.thumbnailUrl}
          <img src={item.thumbnailUrl} alt="" decoding="async" referrerpolicy="no-referrer" />
        {:else}
          <ContentTypeMark kind={item.contentType} />
        {/if}
        <span class="catalog-modal__type">{catalogContentTypeLabel(item.contentType)}</span>
      </div>

      <div class="catalog-modal__content">
        <div class="catalog-modal__heading">
          <div>
            <h2 id="catalog-modal-title">{item.title}</h2>
            <p>{item.creatorName ? `By ${item.creatorName}` : item.provider}</p>
          </div>
        </div>

        <div class="catalog-modal__facts">
          {#if item.publishedAtMs}
            <div><span>Released</span><strong>{formatDate(item.publishedAtMs)}</strong></div>
          {/if}
          {#if item.updatedAtMs && item.updatedAtMs !== item.publishedAtMs}
            <div><span>Updated</span><strong>{formatDate(item.updatedAtMs)}</strong></div>
          {/if}
          {#if item.expectedBytes}
            <div><span>Size</span><strong>{formatBytes(item.expectedBytes)}</strong></div>
          {/if}
        </div>

        {#if item.description}
          <p class="catalog-modal__description">{item.description}</p>
        {/if}

        {#if item.tags.length}
          <div class="catalog-modal__tags" aria-label="Tags">
            {#each item.tags as tag (tag)}<span>{tag}</span>{/each}
          </div>
        {/if}

        {#if onDownload && item.download && item.fileName}
          <div class="catalog-modal__footer">
            <button class="button button--primary" type="button" onclick={() => onDownload?.()} disabled={!canDownload || downloadBusy}>
              {downloadBusy ? "Starting…" : "Download"}
            </button>
          </div>
        {:else if !item.download || !item.fileName}
          <div class="catalog-modal__footer">
            <span class="catalog-modal__unavailable">Download unavailable</span>
          </div>
        {/if}
      </div>
    </section>
  </div>
{/if}
