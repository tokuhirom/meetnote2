<script lang="ts">
    import {Entry} from "./entry.js";
    import {dialog} from "@tauri-apps/api";

    export let entry: Entry;
    export let onDelete: () => void;
    let editingContent : string | undefined = undefined;
    let editMode = false;

    async function enterEditingMode() {
        editMode = true
        editingContent = entry.summary;
    }

    async function saveItem() {
        console.log(`Save file: ${entry}`)
        await entry.save_summary(editingContent!!);
        editMode = false;
    }

    async function regenerateSummaryItem() {
        await entry.regenerateSummary();
    }

    // TODO confirm
    async function deleteItem() {
        // TODO move to menu bar?
        if (await dialog.confirm("Do you want to delete this file?")) {
            await entry.remove();
            onDelete();
        }
    }
</script>

<div>
    <div>
        <button on:click|preventDefault={enterEditingMode}>Edit</button>
        <button on:click|preventDefault={regenerateSummaryItem}>Regenerate Summary</button>
        <button on:click|preventDefault={deleteItem}>Delete</button>
    </div>

    {#if editMode}
        <form on:submit|preventDefault={saveItem}>
            <textarea cols="80" rows="40" bind:value={editingContent}></textarea>
            <button type="submit">Save</button>
        </form>
    {:else}
        {#if entry.summary}
            <pre style="clear: both">{entry.summary}</pre>
        {:else}
            <div class="summary-wip">Summary not available...(WIP?)</div>
        {/if}
    {/if}
</div>

<style>
    .summary-wip {
        margin: 8px;
        color: yellowgreen;
    }
</style>