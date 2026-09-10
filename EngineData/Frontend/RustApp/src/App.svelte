<script lang="ts">
  import { onMount } from "svelte";
  import { RefreshCw } from "@lucide/svelte";
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
  let refreshing = $state(false);
  let snapshot = $state<ProductRuntimeSnapshot | null>(null);

  let health = $derived(snapshot?.backend?.diagnostics.health.state ?? "unknown");
  let runtimeLabel = $derived(
    booting
      ? "Checking runtime"
      : !snapshot?.ready
        ? "Runtime unavailable"
        : health === "degraded"
          ? "Runtime degraded"
          : "Runtime ready",
  );
  let runtimeTone = $derived<"ready" | "warning" | "muted">(
    booting ? "muted" : snapshot?.ready && health !== "degraded" ? "ready" : "warning",
  );

  async function refreshRuntime(): Promise<void> {
    if (refreshing) return;
    refreshing = true;
    snapshot = await runtimeProductFacade.loadProductRuntimeSnapshot();
    booting = false;
    refreshing = false;
  }

  function navigate(next: AppRoute): void {
    route = next;
  }

  onMount(() => {
    void refreshRuntime();
  });
</script>

<div class="app-shell">
  <Sidebar {route} onNavigate={navigate} />

  <main class="app-main">
    <header class="topbar">
      <div>
        <span class="topbar__kicker">SearchNow</span>
        <strong>Bedrock content workspace</strong>
      </div>
      <div class="topbar__actions">
        <button class="icon-button icon-button--quiet" type="button" title="Refresh runtime" aria-label="Refresh runtime" onclick={refreshRuntime} disabled={booting || refreshing}>
          <RefreshCw size={15} class={refreshing ? "spin" : ""} />
        </button>
        <StatusBadge label={runtimeLabel} tone={runtimeTone} />
      </div>
    </header>

    <div class="content-frame">
      {#if booting}
        <section class="page">
          <article class="empty-panel empty-panel--boot">
            <div class="empty-panel__icon"><RefreshCw size={18} class="spin" /></div>
            <div><h2>Starting SearchNow</h2><p>Reading one safe runtime snapshot before opening the workspace.</p></div>
          </article>
        </section>
      {:else if route === "library"}
        <Library runtimeReady={snapshot?.ready ?? false} />
      {:else if route === "discover"}
        <Discover runtimeReady={snapshot?.ready ?? false} providers={snapshot?.backend?.providers ?? []} />
      {:else if route === "downloads"}
        <Downloads runtimeReady={snapshot?.ready ?? false} />
      {:else}
        <Settings {snapshot} />
      {/if}
    </div>
  </main>
</div>
