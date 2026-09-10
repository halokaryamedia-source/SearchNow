<script lang="ts">
  import { AlertTriangle, CheckCircle2, Info } from "@lucide/svelte";

  type NoticeTone = "info" | "warning" | "error" | "success";

  let {
    title,
    message,
    tone = "info",
    actionLabel = null,
    actionDisabled = false,
    onAction = null,
  }: {
    title: string;
    message: string;
    tone?: NoticeTone;
    actionLabel?: string | null;
    actionDisabled?: boolean;
    onAction?: (() => void) | null;
  } = $props();
</script>

<article class={`notice notice--${tone}`} role={tone === "error" ? "alert" : "status"}>
  {#if tone === "success"}
    <CheckCircle2 size={16} aria-hidden="true" />
  {:else if tone === "info"}
    <Info size={16} aria-hidden="true" />
  {:else}
    <AlertTriangle size={16} aria-hidden="true" />
  {/if}
  <div class="notice__body">
    <strong>{title}</strong>
    <span>{message}</span>
  </div>
  {#if actionLabel && onAction}
    <button class="button button--ghost button--compact notice__action" type="button" onclick={onAction} disabled={actionDisabled}>
      {actionLabel}
    </button>
  {/if}
</article>
