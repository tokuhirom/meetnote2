<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {WebviewWindow} from "@tauri-apps/api/window";
    import type {Entry} from "./entry";

    export let onSelectEntry: (entry: Entry) => void;
    export let entry: Entry;


    async function openLog() {
        console.log("open log");
        onSelectEntry(entry);
    }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="file" on:click|preventDefault={openLog} role="navigation">
    <div class="title">{entry.title()}</div>
    {#if entry.summary}
        <pre>{entry.summary}</pre>
    {:else}
        <i>Summary is not available yet.</i>
    {/if}
</div>

<style>
    pre {
        word-break: break-all;
    }
    .title {
        font-weight: bold;
    }
</style>