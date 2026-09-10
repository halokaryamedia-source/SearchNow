<script lang="ts">
  let {
    label,
    value,
    detail,
  }: {
    label: string;
    value: string | number;
    detail: string;
  } = $props();
</script>

<article class="metric-card">
  <span>{label}</span>
  <strong>{value}</strong>
  <small>{detail}</small>
</article>
