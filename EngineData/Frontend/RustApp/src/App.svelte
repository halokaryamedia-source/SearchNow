<script lang="ts">
  import { onMount } from "svelte";
  import { runtimeProductFacade } from "./app/bridge/runtimeProductFacade";
  import type { AppRoute, ProductRuntimeSnapshot } from "./app/shared/types";
  import Sidebar from "./components/layout/Sidebar.svelte";
  import StatusBadge from "./components/ui/StatusBadge.svelte";
  import Discover from "./pages/Discover.svelte";
  import Downloads from "./pages/Downloads.svelte";
  import Library from "./pages/Library.svelte";
  import Settings from "./pages/Settings.svelte";

  let route = $state<AppRoute>("library");
  let booting = $state(true);
  let snapshot = $state<ProductRuntimeSnapshot | null>(null);

  onMount(async () => {
    snapshot = await runtimeProductFacade.loadProductRuntimeSnapshot();
    booting = false;
  });

  function navigate(next: AppRoute): void {
    route = next;
  }
</script>

<div class="app-shell">
  <Sidebar {route} onNavigate={navigate} />

  <main class="app-main">
    <header class="topbar">
      <div>
        <span class="topbar__kicker">SearchNow</span>
        <strong>Content workspace</strong>
      </div>
      <StatusBadge
        label={booting ? "Checking runtime" : snapshot?.ready ? "Runtime ready" : "Runtime unavailable"}
        tone={booting ? "muted" : snapshot?.ready ? "ready" : "warning"}
      />
    </header>

    <div class="content-frame">
      {#if route === "library"}
        <Library runtimeReady={snapshot?.ready ?? false} />
      {:else if route === "discover"}
        <Discover />
      {:else if route === "downloads"}
        <Downloads />
      {:else}
        <Settings {snapshot} />
      {/if}
    </div>
  </main>
</div>
