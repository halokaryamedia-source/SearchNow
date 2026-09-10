<script lang="ts">
  export type ContentDetailItem = {
    label: string;
    value: string | null | undefined;
  };

  let {
    description = null,
    items = [],
    summary = "View details",
  }: {
    description?: string | null;
    items?: ContentDetailItem[];
    summary?: string;
  } = $props();

  let visibleItems = $derived(items.filter((item) => item.value != null && String(item.value).trim().length > 0));
  let hasDescription = $derived(Boolean(description?.trim()));
</script>

{#if hasDescription || visibleItems.length}
  <details class="content-details">
    <summary>{summary}</summary>
    <div class="content-details__body">
      {#if hasDescription}
        <p>{description}</p>
      {/if}
      {#if visibleItems.length}
        <dl>
          {#each visibleItems as item (`${item.label}:${item.value}`)}
            <div>
              <dt>{item.label}</dt>
              <dd>{item.value}</dd>
            </div>
          {/each}
        </dl>
      {/if}
    </div>
  </details>
{/if}
