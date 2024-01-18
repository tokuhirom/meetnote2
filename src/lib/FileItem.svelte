<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {WebviewWindow} from "@tauri-apps/api/window";
    import type {Entry} from "./entry";

    export let onSelectEntry: (entry: Entry) => void;
    export let entry: Entry;

    let editMode = false;
    let editingContent : string | undefined = undefined;

    async function deleteItem() {
        await entry.remove();
    }

    async function enterEditingMode() {
        editMode = true
        editingContent = entry.summary;
    }

    async function openLog() {
        console.log("open log");
        onSelectEntry(entry);
    }

    async function saveItem() {
        console.log(`Save file: ${entry}`)
        await entry.save_summary(editingContent!!);
        editMode = false;
    }

    async function regenerateSummaryItem() {
        await entry.regenerateSummary();
    }
</script>

<div class="file">
    <div>
        <h2 style="float: left;">{entry.title()}</h2>
        <div style="float: right" class="buttons">
            <button on:click|preventDefault={enterEditingMode}>Edit</button>
            <button on:click|preventDefault={openLog}>Log</button>
            <button on:click|preventDefault={deleteItem}>Delete</button>
            <button on:click|preventDefault={regenerateSummaryItem}>Regenerate Summary</button>
        </div>
    </div>
    {#if editMode}
        <form on:submit|preventDefault={saveItem}>
            <textarea cols="80" rows="40" bind:value={editingContent}></textarea>
            <button type="submit">Save</button>
        </form>
    {:else}
        <pre style="clear: both">{entry.summary}</pre>
    {/if}
</div>

<style>
    pre {
        word-break: break-all;
    }
    .file .buttons {
        font-size: x-small;
    }
</style>