<script lang="ts">
  import { RefreshCw, RotateCcw, Search, Trash2, X } from "@lucide/svelte";
  import { runtimeProductFacade } from "../app/bridge/runtimeProductFacade";
  import { downloadStateLabel, formatBytes, formatDateTime, progressPercent } from "../app/shared/format";
  import type { DownloadJob, DownloadManagerSnapshot } from "../app/shared/types";
  import Notice from "../components/ui/Notice.svelte";
  import PageState from "../components/ui/PageState.svelte";
  import ResultsBar from "../components/ui/ResultsBar.svelte";

  type DownloadFilter = "all" | "active" | "completed" | "issues";

  let { runtimeReady, active }: { runtimeReady: boolean; active: boolean } = $props();
  let snapshot = $state<DownloadManagerSnapshot | null>(null);
  let loading = $state(false);
  let error = $state("");
  let actionJobId = $state<string | null>(null);
  let query = $state("");
  let filter = $state<DownloadFilter>("all");

  let jobs = $derived((snapshot?.jobs ?? []).slice().sort((a, b) => b.updatedAtMs - a.updatedAtMs));
  let visibleJobs = $derived(jobs.filter((job) => matchesFilter(job)));
  let hasActivity = $derived((snapshot?.activeJobs ?? 0) > 0 || (snapshot?.queuedJobs ?? 0) > 0);
  let controlsChanged = $derived(query.trim().length > 0 || filter !== "all");

  function matchesFilter(job: DownloadJob): boolean {
    if (filter === "active" && !["queued", "preparing", "transferring", "finalizing", "cancelRequested"].includes(job.state)) return false;
    if (filter === "completed" && job.state !== "completed") return false;
    if (filter === "issues" && !["failed", "interrupted", "cancelled"].includes(job.state)) return false;
    const needle = query.trim().toLowerCase();
    if (!needle) return true;
    return `${job.displayName} ${job.destinationFileName} ${job.state}`.toLowerCase().includes(needle);
  }

  function resetControls(): void {
    query = "";
    filter = "all";
  }

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
    if (!active || !runtimeReady) return;
    let disposed = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    const poll = async (): Promise<void> => {
      await refresh(false);
      if (!disposed && active) timer = setTimeout(poll, hasActivity ? 1200 : 5000);
    };
    void poll();

    return () => {
      disposed = true;
      if (timer) clearTimeout(timer);
    };
  });
</script>

<section class="page" hidden={!active}>
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

  {#if snapshot && jobs.length > 0}
    <div class="toolbar">
      <label class="search-field">
        <Search size={15} aria-hidden="true" />
        <input bind:value={query} type="search" placeholder="Search downloads" aria-label="Search download history" />
      </label>
      <select class="select-field" bind:value={filter} aria-label="Filter download history">
        <option value="all">All downloads</option>
        <option value="active">Active</option>
        <option value="completed">Completed</option>
        <option value="issues">Needs attention</option>
      </select>
    </div>
    <ResultsBar
      label={`${visibleJobs.length} of ${jobs.length} download${jobs.length === 1 ? "" : "s"}`}
      detail={controlsChanged ? "Current search and history filter are applied." : "Showing the complete retained download history."}
      showReset={controlsChanged}
      resetLabel="Reset history view"
      onReset={resetControls}
    />
  {/if}

  {#if snapshot?.schedulerError}
    <Notice tone="warning" title="Download queue needs attention." message={snapshot.schedulerError.message} />
  {/if}

  {#if error}
    <Notice tone="error" title="Download action failed." message={error} />
  {/if}

  {#if !runtimeReady}
    <PageState marker="03" title="Downloads unavailable" message="SearchNow could not connect to the desktop runtime needed to manage transfers." />
  {:else if !snapshot && !error}
    <PageState kind="loading" title="Loading downloads" message="Reading your current queue and recent download history." />
  {:else if snapshot && jobs.length === 0}
    <PageState marker="03" title="No downloads yet" message="Downloads started from Discover will appear here with their current state and progress." />
  {:else if snapshot && visibleJobs.length === 0}
    <PageState
      marker="03"
      title="No matching downloads"
      message="Reset or change the current search and history filter to see other download jobs."
      actionLabel={controlsChanged ? "Reset history view" : null}
      onAction={controlsChanged ? resetControls : null}
    />
  {:else if snapshot}
    <div class="download-list" aria-live="polite">
      {#each visibleJobs as job (job.id)}
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

            <div class:progress-track--indeterminate={percent === null && ["preparing", "transferring", "finalizing"].includes(job.state)} class="progress-track" aria-label={`${job.displayName} progress`}>
              {#if percent !== null}<span style={`width:${percent}%`}></span>{:else}<span></span>{/if}
            </div>

            <div class="download-card__meta">
              <span>{formatBytes(job.progress.downloadedBytes)}{job.progress.totalBytes !== null ? ` / ${formatBytes(job.progress.totalBytes)}` : ""}</span>
              <span>{percent !== null ? `${percent}%` : `Attempt ${job.attempt}`}</span>
            </div>

            {#if job.lastError}
              <Notice tone="warning" title={job.lastError.code} message={job.lastError.message} />
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
                <X size={16} aria-hidden="true" />
              </button>
            {/if}
            {#if canRetry(job)}
              <button class="icon-button" type="button" title="Retry download" aria-label={`Retry ${job.displayName}`} onclick={() => retry(job)} disabled={actionJobId === job.id}>
                <RotateCcw size={16} aria-hidden="true" />
              </button>
            {/if}
            {#if canRemove(job)}
              <button class="icon-button" type="button" title="Remove from history" aria-label={`Remove ${job.displayName} from history`} onclick={() => remove(job)} disabled={actionJobId === job.id}>
                <Trash2 size={16} aria-hidden="true" />
              </button>
            {/if}
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>
