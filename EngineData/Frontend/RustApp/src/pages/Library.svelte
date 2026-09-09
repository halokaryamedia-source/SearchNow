<script lang="ts">
  let { runtimeReady }: { runtimeReady: boolean } = $props();
</script>

<section class="page">
  <div class="page-heading">
    <div>
      <span class="eyebrow">My content</span>
      <h1>Library</h1>
      <p>Installed and local Minecraft Bedrock content will live here.</p>
    </div>
  </div>

  <div class="metric-grid">
    <article class="metric-card"><span>Installed</span><strong>0</strong><small>Not scanned yet</small></article>
    <article class="metric-card"><span>Local packs</span><strong>0</strong><small>Not scanned yet</small></article>
    <article class="metric-card"><span>Projects</span><strong>0</strong><small>Reserved for local work</small></article>
  </div>

  <article class="empty-panel">
    <div class="empty-panel__icon">01</div>
    <div>
      <h2>Library engine comes next</h2>
      <p>{runtimeReady ? "The desktop runtime is connected. The next vertical slice will detect Minecraft and build the local library." : "The desktop runtime is not connected yet."}</p>
    </div>
  </article>
</section>
