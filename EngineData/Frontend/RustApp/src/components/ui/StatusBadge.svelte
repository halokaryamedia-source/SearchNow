<script lang="ts">
  type Tone = "ready" | "warning" | "muted";

  let { label, tone = "muted" }: { label: string; tone?: Tone } = $props();
</script>

<span class={`status-badge status-badge--${tone}`}>
  <span class="status-badge__dot" aria-hidden="true"></span>
  {label}
</span>
