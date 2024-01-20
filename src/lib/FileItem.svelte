<script lang="ts">
    import type {Entry} from "./entry";
    import type {PostProcessStatus} from "./postprocess";

    export let postProcessingStatus: PostProcessStatus | undefined;
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
    {#if postProcessingStatus && postProcessingStatus.path === entry.path}
        <div class="now-postprocessing">{postProcessingStatus.message}</div>
    {:else if recordingEntry && recordingEntry.path === entry.path}
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

    .now-postprocessing {
        color: #396cd8;
    }
    .now-recording {
        color: red;
    }
</style>
