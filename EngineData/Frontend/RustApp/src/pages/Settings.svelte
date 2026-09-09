<script lang="ts">
  import type { ProductRuntimeSnapshot } from "../app/shared/types";

  let { snapshot }: { snapshot: ProductRuntimeSnapshot | null } = $props();
</script>

<section class="page">
  <div class="page-heading">
    <div>
      <span class="eyebrow">Application</span>
      <h1>Settings</h1>
      <p>Persistent settings will remain runtime-owned and local-first.</p>
    </div>
  </div>

  <div class="settings-grid">
    <article class="settings-card">
      <span class="settings-card__label">Desktop runtime</span>
      <strong>{snapshot?.ready ? "Connected" : "Unavailable"}</strong>
      <p>{snapshot?.summary ?? "Checking runtime..."}</p>
    </article>
    <article class="settings-card">
      <span class="settings-card__label">Backend</span>
      <strong>{snapshot?.runtime?.backend ?? "Rust"}</strong>
      <p>No Python worker is part of the current architecture.</p>
    </article>
    <article class="settings-card">
      <span class="settings-card__label">Platform</span>
      <strong>{snapshot?.runtime?.platform ?? "Windows target"}</strong>
      <p>{snapshot?.runtime ? `${snapshot.runtime.architecture} · app ${snapshot.runtime.appVersion}` : "Runtime details appear when Tauri is connected."}</p>
    </article>
  </div>
</section>
