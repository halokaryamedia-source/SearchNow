<script lang="ts">
  import { AlertTriangle, CheckCircle2, Info } from "@lucide/svelte";

  type NoticeTone = "info" | "warning" | "error" | "success";

  let {
    title,
    message,
    tone = "info",
  }: {
    title: string;
    message: string;
    tone?: NoticeTone;
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
  <div>
    <strong>{title}</strong>
    <span>{message}</span>
  </div>
</article>
