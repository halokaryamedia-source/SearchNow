<script lang="ts">
  export type TechnicalDetailItem = {
    label: string;
    value: string | null | undefined;
  };

  let {
    items,
    summary = "Technical details",
  }: {
    items: TechnicalDetailItem[];
    summary?: string;
  } = $props();

  let visibleItems = $derived(items.filter((item) => item.value != null && String(item.value).trim().length > 0));
</script>

{#if visibleItems.length}
  <details class="technical-details">
    <summary>{summary}</summary>
    <dl>
      {#each visibleItems as item (`${item.label}:${item.value}`)}
        <div>
          <dt>{item.label}</dt>
          <dd>{item.value}</dd>
        </div>
      {/each}
    </dl>
  </details>
{/if}
