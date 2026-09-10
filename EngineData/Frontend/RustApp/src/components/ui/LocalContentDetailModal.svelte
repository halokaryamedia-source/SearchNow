<script lang="ts">
  import { FolderOpen, X } from "@lucide/svelte";
  import { localContentTypeLabel } from "../../app/shared/format";
  import type { LocalContentItem } from "../../app/shared/types";
  import ContentTypeMark from "./ContentTypeMark.svelte";
  import TechnicalDetails from "./TechnicalDetails.svelte";

  let {
    item,
    open,
    onClose,
    onOpenFolder,
    actionBusy = false,
  }: {
    item: LocalContentItem | null;
    open: boolean;
    onClose: () => void;
    onOpenFolder: () => void;
    actionBusy?: boolean;
  } = $props();

  function handleBackdrop(event: MouseEvent): void {
    if (event.target === event.currentTarget && !actionBusy) onClose();
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (open && event.key === "Escape" && !actionBusy) onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && item}
  <div class="catalog-modal__backdrop" role="presentation" onclick={handleBackdrop}>
    <div class="catalog-modal" role="dialog" aria-modal="true" aria-labelledby="library-modal-title">
      <button class="catalog-modal__close" type="button" aria-label="Close details" onclick={onClose} disabled={actionBusy}>
        <X size={18} aria-hidden="true" />
      </button>

      <div class="catalog-modal__media catalog-modal__media--local">
        <ContentTypeMark kind={item.contentType} />
        <span class="catalog-modal__type">{localContentTypeLabel(item.contentType)}</span>
      </div>

      <div class="catalog-modal__content">
        <div class="catalog-modal__heading">
          <div>
            <h2 id="library-modal-title">{item.title}</h2>
            <p>{item.status === "ready" ? "Ready" : "Needs review"}</p>
          </div>
        </div>

        <div class="catalog-modal__facts">
          <div><span>Type</span><strong>{localContentTypeLabel(item.contentType)}</strong></div>
          {#if item.version.length}<div><span>Version</span><strong>v{item.version.join(".")}</strong></div>{/if}
          {#if item.isDevelopment}<div><span>Source</span><strong>Development folder</strong></div>{/if}
        </div>

        {#if item.description}
          <p class="catalog-modal__description">{item.description}</p>
        {/if}

        {#if item.issue}
          <div class="catalog-modal__issue">
            <strong>Needs review</strong>
            <span>{item.issue}</span>
          </div>
        {/if}

        <TechnicalDetails
          items={[
            { label: "Location", value: item.path },
            { label: "Storage root", value: item.rootId },
            { label: "Manifest UUID", value: item.manifestUuid },
          ]}
        />

        <div class="catalog-modal__footer">
          <button class="button button--secondary" type="button" onclick={onOpenFolder} disabled={actionBusy}>
            <FolderOpen size={15} aria-hidden="true" />
            {actionBusy ? "Opening…" : "Open folder"}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
