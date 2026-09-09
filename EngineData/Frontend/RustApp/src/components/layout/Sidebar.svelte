<script lang="ts">
  import { Download, Library, Search, Settings } from "@lucide/svelte";
  import type { Component } from "svelte";
  import type { AppRoute } from "../../app/shared/types";

  let {
    route,
    onNavigate,
  }: {
    route: AppRoute;
    onNavigate: (route: AppRoute) => void;
  } = $props();

  const items: { id: AppRoute; label: string; icon: Component }[] = [
    { id: "library", label: "Library", icon: Library },
    { id: "discover", label: "Discover", icon: Search },
    { id: "downloads", label: "Downloads", icon: Download },
    { id: "settings", label: "Settings", icon: Settings },
  ];
</script>

<aside class="sidebar">
  <div class="brand">
    <div class="brand__mark">S</div>
    <div>
      <strong>SearchNow</strong>
      <span>Bedrock content manager</span>
    </div>
  </div>

  <nav class="sidebar__nav" aria-label="Primary navigation">
    {#each items as item}
      <button
        class:nav-item--active={route === item.id}
        class="nav-item"
        type="button"
        onclick={() => onNavigate(item.id)}
      >
        <item.icon size={18} strokeWidth={1.8} />
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="sidebar__footer">
    <span>Local-first architecture</span>
    <small>v0.1 foundation</small>
  </div>
</aside>
