<script lang="ts">
  import { Download, Library, Search, Settings } from "@lucide/svelte";
  import type { Component } from "svelte";
  import { APP_ROUTES } from "../../app/shared/navigation";
  import type { AppRoute } from "../../app/shared/types";

  let {
    route,
    onNavigate,
  }: {
    route: AppRoute;
    onNavigate: (route: AppRoute) => void;
  } = $props();

  const icons: Record<AppRoute, Component> = {
    library: Library,
    discover: Search,
    downloads: Download,
    settings: Settings,
  };
</script>

<aside class="sidebar">
  <div class="brand">
    <div class="brand__mark" aria-hidden="true">S</div>
    <strong>SearchNow</strong>
  </div>

  <nav class="sidebar__nav" aria-label="Primary navigation">
    {#each APP_ROUTES as item}
      {@const Icon = icons[item.id]}
      <button
        class:nav-item--active={route === item.id}
        class="nav-item"
        type="button"
        aria-current={route === item.id ? "page" : undefined}
        title={item.label}
        onclick={() => onNavigate(item.id)}
      >
        <Icon size={18} strokeWidth={1.8} aria-hidden="true" />
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>
</aside>
