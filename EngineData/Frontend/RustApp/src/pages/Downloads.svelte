<script lang="ts">
  import { AlertTriangle, RefreshCw, RotateCcw, Trash2, X } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import { downloadStateLabel, formatBytes, formatDateTime, progressPercent } from "../app/shared/format";
  import type { DownloadJob, DownloadManagerSnapshot } from "../app/shared/types";

  let { runtimeReady }: { runtimeReady: boolean } = $props();
  let snapshot = $state<DownloadManagerSnapshot | null>(null);
  let loading = $state(false);
  let error = $state("");
  let actionJobId = $state<string | null>(null);

  let jobs = $derived((snapshot?.jobs ?? []).slice().sort((a, b) => b.updatedAtMs - a.updatedAtMs));
  let hasActivity = $derived((snapshot?.activeJobs ?? 0) > 0 || (snapshot?.queuedJobs ?? 0) > 0);

  function canCancel(job: DownloadJob): boolean {
    return ["queued", "preparing", "transferring"].includes(job.state);
  }

  function canRetry(job: DownloadJob): boolean {
    if (job.state === "failed") return job.lastError?.retryable ?? false;
    return job.state === "cancelled" || job.state === "interrupted";
  }

  function canRemove(job: DownloadJob): boolean {
    return job.state === "completed" || job.state === "failed" || job.state === "cancelled";
  }

  async function refresh(showBusy = true): Promise<void> {
    if (!runtimeReady || loading) return;
    if (showBusy) loading = true;
    const result = await runtimeProductFacade.loadDownloads();
    if (result.ok) {
      snapshot = result.data;
      error = "";
    } else {
      error = result.error.message;
    }
    loading = false;
  }

  async function cancel(job: DownloadJob): Promise<void> {
    actionJobId = job.id;
    const result = await runtimeProductFacade.cancelDownload(job.id);
    if (!result.ok) error = result.error.message;
    await refresh(false);
    actionJobId = null;
  }

  async function retry(job: DownloadJob): Promise<void> {
    actionJobId = job.id;
    const result = await runtimeProductFacade.retryDownload(job.id);
    if (!result.ok) error = result.error.message;
    await refresh(false);
    actionJobId = null;
  }

  async function remove(job: DownloadJob): Promise<void> {
    actionJobId = job.id;
    const result = await runtimeProductFacade.removeDownload(job.id);
    if (result.ok) {
      snapshot = result.data;
      error = "";
    } else {
      error = result.error.message;
    }
    actionJobId = null;
  }

  $effect(() => {
    if (!runtimeReady) return;
    let disposed = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    const poll = async (): Promise<void> => {
      await refresh(false);
      if (!disposed) timer = setTimeout(poll, hasActivity ? 1200 : 5000);
    };
    void poll();

    return () => {
      disposed = true;
      if (timer) clearTimeout(timer);
    };
  });
</script>

<section class="page">
  <div class="page-heading page-heading--actions">
    <div>
      <span class="eyebrow">Transfer queue</span>
      <h1>Downloads</h1>
      <p>Track active downloads, retry interrupted transfers, and manage completed history.</p>
    </div>
    <button class="button button--secondary" type="button" onclick={() => refresh()} disabled={!runtimeReady || loading}>
      <RefreshCw size={15} class={loading ? "spin" : ""} />
      Refresh
    </button>
  </div>

  <div class="metric-grid">
    <article class="metric-card"><span>Active</span><strong>{snapshot?.activeJobs ?? "—"}</strong><small>Maximum {snapshot?.policy.maxActive ?? "—"} concurrent</small></article>
    <article class="metric-card"><span>Queued</span><strong>{snapshot?.queuedJobs ?? "—"}</strong><small>Waiting for an execution slot</small></article>
    <article class="metric-card"><span>History</span><strong>{snapshot?.jobs.length ?? "—"}</strong><small>Maximum {snapshot?.policy.maxJobs ?? "—"} retained jobs</small></article>
  </div>

  {#if snapshot?.schedulerError}
    <article class="notice notice--warning">
      <AlertTriangle size={16} />
      <div><strong>Download queue needs attention.</strong><span>{snapshot.schedulerError.message}</span></div>
    </article>
  {/if}

  {#if error}
    <article class="notice notice--error">
      <AlertTriangle size={16} />
      <div><strong>Download action failed.</strong><span>{error}</span></div>
    </article>
  {/if}

  {#if !runtimeReady}
    <article class="empty-panel">
      <div class="empty-panel__icon">03</div>
      <div><h2>Downloads unavailable</h2><p>SearchNow could not connect to the desktop runtime needed to manage transfers.</p></div>
    </article>
  {:else if !snapshot && !error}
    <article class="empty-panel">
      <div class="empty-panel__icon"><RefreshCw size={18} class="spin" /></div>
      <div><h2>Loading downloads</h2><p>Reading your current queue and recent download history.</p></div>
    </article>
  {:else if snapshot && jobs.length === 0}
    <article class="empty-panel">
      <div class="empty-panel__icon">03</div>
      <div><h2>No downloads yet</h2><p>Downloads started from Discover will appear here with their current state and progress.</p></div>
    </article>
  {:else if snapshot}
    <div class="download-list">
      {#each jobs as job (job.id)}
        {@const percent = progressPercent(job.progress.downloadedBytes, job.progress.totalBytes)}
        <article class="download-card">
          <div class="download-card__main">
            <div class="download-card__heading">
              <div>
                <span class={`state-pill state-pill--${job.state}`}>{downloadStateLabel(job.state)}</span>
                <h2>{job.displayName}</h2>
              </div>
              <span class="download-card__time">{formatDateTime(job.updatedAtMs)}</span>
            </div>

            <div class:progress-track--indeterminate={percent === null && ["preparing", "transferring", "finalizing"].includes(job.state)} class="progress-track">
              {#if percent !== null}<span style={`width:${percent}%`}></span>{:else}<span></span>{/if}
            </div>

            <div class="download-card__meta">
              <span>{formatBytes(job.progress.downloadedBytes)}{job.progress.totalBytes !== null ? ` / ${formatBytes(job.progress.totalBytes)}` : ""}</span>
              <span>{percent !== null ? `${percent}%` : `Attempt ${job.attempt}`}</span>
            </div>

            {#if job.lastError}
              <div class="download-card__error"><AlertTriangle size={14} /><span>{job.lastError.message}</span></div>
            {/if}

            <details class="technical-details">
              <summary>Technical details</summary>
              <dl>
                <div><dt>Output</dt><dd>{job.destinationFileName}</dd></div>
                <div><dt>Job</dt><dd>{job.id}</dd></div>
                <div><dt>Transport</dt><dd>{job.source.transport}</dd></div>
              </dl>
            </details>
          </div>

          <div class="download-card__actions">
            {#if canCancel(job)}
              <button class="icon-button" type="button" title="Cancel download" aria-label={`Cancel ${job.displayName}`} onclick={() => cancel(job)} disabled={actionJobId === job.id}>
                <X size={16} />
              </button>
            {/if}
            {#if canRetry(job)}
              <button class="icon-button" type="button" title="Retry download" aria-label={`Retry ${job.displayName}`} onclick={() => retry(job)} disabled={actionJobId === job.id}>
                <RotateCcw size={16} />
              </button>
            {/if}
            {#if canRemove(job)}
              <button class="icon-button" type="button" title="Remove from history" aria-label={`Remove ${job.displayName} from history`} onclick={() => remove(job)} disabled={actionJobId === job.id}>
                <Trash2 size={16} />
              </button>
            {/if}
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
