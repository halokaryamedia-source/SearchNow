<script lang="ts">
  let {
    label,
    detail = null,
    resetLabel = "Reset",
    showReset = false,
    onReset = null,
  }: {
    label: string;
    detail?: string | null;
    resetLabel?: string;
    showReset?: boolean;
    onReset?: (() => void) | null;
  } = $props();
</script>

<div class="results-bar" aria-live="polite">
  <div class="results-bar__copy">
    <strong>{label}</strong>
    {#if detail}<span>{detail}</span>{/if}
  </div>
  {#if showReset && onReset}
    <button class="button button--ghost button--compact" type="button" onclick={onReset}>{resetLabel}</button>
  {/if}
</div>
