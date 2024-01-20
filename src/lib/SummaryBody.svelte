<script lang="ts">
    import {Entry} from "./entry.js";
    import {marked} from "marked";
    import DOMPurify from "dompurify";
    import StatusIndicator from "./StatusIndicator.svelte";
    import {dialog} from "@tauri-apps/api";

    export let entry: Entry;
    export let onDelete: () => void;

    let editingContent : string | undefined = undefined;
    let editMode = false;

    async function regenerateSummaryItem() {
        try {
            await entry.regenerateSummary();
            await entry.readSummary();
            entry = entry;
        } catch (e) {
            console.log(e);
            await dialog.message(`${e}`);
        }
    }

    async function deleteItem() {
        // TODO move to menu bar?
        if (await dialog.confirm("Do you want to delete this file?")) {
            await entry.remove();
            onDelete();
        }
    }
    async function enterEditingMode() {
        editMode = true
        editingContent = entry.summary;
    }

    async function saveItem() {
        console.log(`Save file: ${entry}`)
        await entry.saveSummary(editingContent!!);
        editMode = false;
    }

    function markdown(src: string) {
        return DOMPurify.sanitize(marked.parse(src));
    }
</script>

<div>
    <div>
        <button on:click|preventDefault={regenerateSummaryItem}>Regenerate Summary</button>
        <button on:click|preventDefault={deleteItem}>Delete</button>
    </div>

    <hr/>

    <StatusIndicator entry={entry} />

    <hr/>

    <button on:click|preventDefault={enterEditingMode}>Edit</button>

    {#if editMode}
        <form on:submit|preventDefault={saveItem}>
            <textarea cols="80" rows="40" bind:value={editingContent}></textarea>
            <button type="submit">Save</button>
        </form>
    {:else}
        {#if entry.summary}
            <div class="summary">{@html markdown(entry.summary)}</div>
        {:else}
            <div class="summary-wip">Summary not available...(WIP?)</div>
        {/if}
    {/if}

    <div class="path">{entry.path}</div>
</div>

<style>
    .path {
        color: darkslategray;
        font-size: 80%;
        background: none;
    }

    .summary-wip {
        margin: 8px;
        color: yellowgreen;
    }
</style>
