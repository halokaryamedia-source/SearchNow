<script lang="ts">
  let {
    label,
    state,
  }: {
    label: string;
    state: string;
  } = $props();
</script>

<span class={`state-pill state-pill--${state}`} aria-label={label}>{label}</span>
