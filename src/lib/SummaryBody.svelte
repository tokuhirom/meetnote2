<script lang="ts">
    import {Entry} from "./entry.js";
    import {marked} from "marked";
    import DOMPurify from "dompurify";
    import StatusIndicator from "./StatusIndicator.svelte";
    import {dialog} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";

    export let entry: Entry;
    export let recordingEntry: Entry | undefined;
    export let onDelete: () => void;

    let editingContent : string | undefined = undefined;
    let editMode = false;

    listen("do_edit_summary", () => {
        console.log("Called do_edit_summary");
        enterEditingMode();
    });
    listen("do_delete_entry", () => {
        console.log("Called do_delete_entry");
        deleteItem();
    });

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
        if (await dialog.confirm(`Do you want to delete this file?'\n\n${entry.summary?.replace(/([*#])+/, '').slice(0, 30)}`)) {
            await entry.remove();
            console.log("deleted file");
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
    </div>

    <hr/>

    <StatusIndicator entry={entry} />

    <hr/>

    {#if editMode}
        <form on:submit|preventDefault={saveItem}>
            <textarea cols="80" rows="40" bind:value={editingContent}></textarea>
            <button type="submit">Save</button>
        </form>
    {:else}
        {#if recordingEntry && recordingEntry.path === entry.path}
            <div class="now-recording">Now recording this entry...</div>
        {:else if entry.summary}
            <div class="summary">{@html markdown(entry.summary)}</div>
        {:else}
            <div class="summary-wip">Summary not available...(WIP?)
                <!--{#if await entry.hasMicWav()}-->
<!--                    TODO: implement run postprocess button.-->
<!--                {:else}-->
<!--                    TODO: implement run postprocess button.-->
<!--                {/if}-->
            </div>
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

    .now-recording {
        color: red;
    }
</style>