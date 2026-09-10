<script lang="ts">
  import { Blocks, Box, Globe2, Package, Palette, UserRound } from "@lucide/svelte";

  type ContentMarkKind =
    | "world"
    | "behaviorPack"
    | "resourcePack"
    | "skinPack"
    | "addon"
    | "skin"
    | "persona"
    | "other";

  let { kind }: { kind: ContentMarkKind } = $props();
</script>

<span class="content-type-mark" aria-hidden="true">
  {#if kind === "world"}
    <Globe2 size={22} strokeWidth={1.7} />
  {:else if kind === "behaviorPack" || kind === "addon"}
    <Blocks size={22} strokeWidth={1.7} />
  {:else if kind === "resourcePack"}
    <Palette size={22} strokeWidth={1.7} />
  {:else if kind === "skinPack" || kind === "skin" || kind === "persona"}
    <UserRound size={22} strokeWidth={1.7} />
  {:else if kind === "other"}
    <Package size={22} strokeWidth={1.7} />
  {:else}
    <Box size={22} strokeWidth={1.7} />
  {/if}
</span>
