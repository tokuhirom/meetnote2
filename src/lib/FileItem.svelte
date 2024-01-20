<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {WebviewWindow} from "@tauri-apps/api/window";
    import type {Entry} from "./entry";

    export let recordingEntry: Entry | undefined;
    export let onSelectEntry: (entry: Entry) => void;
    export let entry: Entry;

    async function openLog() {
        console.log("open log");
        onSelectEntry(entry);
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="file" on:click|preventDefault={openLog} role="navigation">
    <div class="timestamp">{entry.title()}</div>
    {#if recordingEntry && recordingEntry.path === entry.path}
        <div class="now-recording">Now recording this entry...</div>
    {:else if entry.summary}
        <div class="summary">{entry.summary.replace(/#+/g, '')}</div>
    {:else}
        <i>Summary is not available yet.</i>
    {/if}
</div>

<style>
    .timestamp {
        color: dimgray;
        font-size: 80%;
    }
    .summary {
        overflow: hidden;
        white-space: nowrap;
        text-overflow: clip;
    }
    .file {
        border-bottom: dimgray 1px solid;
        clear: both;
    }

    .now-recording {
        color: red;
    }
</style>
