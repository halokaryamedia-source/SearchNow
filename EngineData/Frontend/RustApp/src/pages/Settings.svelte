<script lang="ts">
  import { AlertTriangle, Check, RefreshCw, Save } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import type { AppSettings, MinecraftDiscoverySnapshot, ProductRuntimeSnapshot } from "../app/shared/types";
  import DiagnosticsPanel from "../components/settings/DiagnosticsPanel.svelte";

  let { snapshot }: { snapshot: ProductRuntimeSnapshot | null } = $props();
  let schemaVersion = $state(1);
  let rootOverride = $state("");
  let includePreview = $state(false);
  let includeLegacyUwp = $state(true);
  let includeDevelopmentContent = $state(false);
  let discovery = $state<MinecraftDiscoverySnapshot | null>(snapshot?.backend?.minecraft ?? null);
  let baselineSettings = $state<AppSettings | null>(null);
  let loading = $state(false);
  let loaded = $state(false);
  let saving = $state(false);
  let scanning = $state(false);
  let error = $state("");
  let saved = $state(false);

  let dirty = $derived(
    baselineSettings !== null &&
      (rootOverride.trim() !== (baselineSettings.minecraft.rootOverride ?? "") ||
        includePreview !== baselineSettings.minecraft.includePreview ||
        includeLegacyUwp !== baselineSettings.minecraft.includeLegacyUwp ||
        includeDevelopmentContent !== baselineSettings.minecraft.includeDevelopmentContent),
  );

  function applySettings(settings: AppSettings): void {
    schemaVersion = settings.schemaVersion;
    rootOverride = settings.minecraft.rootOverride ?? "";
    includePreview = settings.minecraft.includePreview;
    includeLegacyUwp = settings.minecraft.includeLegacyUwp;
    includeDevelopmentContent = settings.minecraft.includeDevelopmentContent;
    baselineSettings = settings;
  }

  async function load(): Promise<void> {
    if (!snapshot?.ready || loading) return;
    loading = true;
    error = "";
    const result = await runtimeProductFacade.loadSettings();
    if (result.ok) applySettings(result.data);
    else error = result.error.message;
    loaded = true;
    loading = false;
  }

  async function save(): Promise<void> {
    if (!snapshot?.ready || saving || !dirty) return;
    saving = true;
    saved = false;
    error = "";
    const result = await runtimeProductFacade.saveSettings({
      schemaVersion,
      minecraft: {
        rootOverride: rootOverride.trim() || null,
        includePreview,
        includeLegacyUwp,
        includeDevelopmentContent,
      },
    });
    if (result.ok) {
      applySettings(result.data);
      saved = true;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function rescan(): Promise<void> {
    if (!snapshot?.ready || scanning || dirty) return;
    scanning = true;
    error = "";
    const result = await runtimeProductFacade.discoverMinecraft();
    if (result.ok) discovery = result.data;
    else error = result.error.message;
    scanning = false;
  }

  $effect(() => {
    if (!snapshot?.ready) {
      loaded = false;
      return;
    }
    if (!loaded && !loading) void load();
  });

  $effect(() => {
    if (dirty) saved = false;
  });
</script>

<section class="page">
  <div class="page-heading page-heading--actions">
    <div>
      <span class="eyebrow">Application</span>
      <h1>Settings</h1>
      <p>Manage Minecraft discovery preferences and review the local desktop runtime.</p>
    </div>
    <button class="button button--primary" type="button" onclick={save} disabled={!snapshot?.ready || loading || saving || !dirty}>
      {#if saved && !saving}<Check size={15} />{:else}<Save size={15} />{/if}
      {saving ? "Saving" : saved ? "Saved" : "Save settings"}
    </button>
  </div>

  {#if error}
    <article class="notice notice--error">
      <AlertTriangle size={16} />
      <div><strong>Settings action failed.</strong><span>{error}</span></div>
    </article>
  {/if}

  <div class="settings-layout">
    <div class="settings-stack">
      <article class="settings-section">
        <div class="settings-section__heading">
          <div><span class="eyebrow">Minecraft</span><h2>Content discovery</h2></div>
          <button
            class="button button--secondary button--compact"
            type="button"
            title={dirty ? "Save settings before rescanning" : "Rescan Minecraft locations"}
            onclick={rescan}
            disabled={!snapshot?.ready || scanning || dirty}
          >
            <RefreshCw size={14} class={scanning ? "spin" : ""} />Rescan
          </button>
        </div>

        <label class="field">
          <span>Manual Minecraft data location</span>
          <input bind:value={rootOverride} type="text" placeholder="Leave empty to use automatic detection" disabled={!snapshot?.ready || loading} />
          <small>Optional override. Leave empty to use automatic Minecraft Bedrock discovery.</small>
        </label>

        <div class="toggle-list">
          <label class="toggle-row">
            <div><strong>Include Minecraft Preview</strong><span>Scan Preview storage in addition to the stable installation.</span></div>
            <input bind:checked={includePreview} type="checkbox" disabled={!snapshot?.ready || loading} />
          </label>
          <label class="toggle-row">
            <div><strong>Include legacy UWP locations</strong><span>Keep legacy Windows Bedrock locations as fallback candidates.</span></div>
            <input bind:checked={includeLegacyUwp} type="checkbox" disabled={!snapshot?.ready || loading} />
          </label>
          <label class="toggle-row">
            <div><strong>Include development content</strong><span>Index development behavior, resource, and skin-pack folders.</span></div>
            <input bind:checked={includeDevelopmentContent} type="checkbox" disabled={!snapshot?.ready || loading} />
          </label>
        </div>
      </article>

      <article class="settings-section">
        <div class="settings-section__heading">
          <div><span class="eyebrow">Detected storage</span><h2>Minecraft Bedrock</h2></div>
          <span class={`state-pill state-pill--${discovery?.state === "found" ? "completed" : "interrupted"}`}>
            {discovery?.state === "found" ? "Detected" : discovery?.state === "unsupportedPlatform" ? "Unsupported" : "Not detected"}
          </span>
        </div>
        <p class="section-copy">{discovery?.message ?? "Minecraft discovery information is not available yet."}</p>

        {#if discovery?.roots.length}
          <div class="root-list">
            {#each discovery.roots as root (root.id)}
              <div class="root-row">
                <div><strong>{root.channel === "preview" ? "Minecraft Preview" : root.channel === "custom" ? "Custom location" : "Minecraft Bedrock"}</strong><span>{root.storageKind}</span></div>
                <code>{root.root}</code>
              </div>
            {/each}
          </div>
        {/if}
      </article>

      <DiagnosticsPanel runtimeReady={snapshot?.ready ?? false} />
    </div>

    <aside class="settings-stack">
      <article class="settings-card settings-card--large">
        <span class="settings-card__label">Desktop runtime</span>
        <strong>{snapshot?.ready ? "Connected" : "Unavailable"}</strong>
        <p>{snapshot?.summary ?? "Checking runtime..."}</p>
      </article>
      <article class="settings-card settings-card--large">
        <span class="settings-card__label">Backend</span>
        <strong>{snapshot?.runtime?.backend ?? "Rust"}</strong>
        <p>{snapshot?.runtime ? `${snapshot.runtime.platform} · ${snapshot.runtime.architecture} · app ${snapshot.runtime.appVersion}` : "Runtime details appear when Tauri is connected."}</p>
      </article>
      <article class="settings-card settings-card--large">
        <span class="settings-card__label">Runtime health</span>
        <strong>{snapshot?.backend?.diagnostics.health.state ?? "Unknown"}</strong>
        <p>{snapshot?.backend ? `${snapshot.backend.diagnostics.health.errorEvents} retained errors · ${snapshot.backend.diagnostics.health.warningEvents} warnings` : "Diagnostics are runtime-owned and remain local."}</p>
      </article>
    </aside>
  </div>
</section>
