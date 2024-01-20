<script lang="ts">
    import {dialog} from "@tauri-apps/api";
    import StatusIndicator from "./StatusIndicator.svelte";
    import SummaryBody from "./SummaryBody.svelte";
    import {Entry} from "./entry";

    export let onDelete: () => void;

    export let entry: Entry;

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
</script>

<div>
    <div>
        <button on:click|preventDefault={regenerateSummaryItem}>Regenerate Summary</button>
        <button on:click|preventDefault={deleteItem}>Delete</button>
    </div>

    <hr/>

    <StatusIndicator entry={entry} />

    <hr/>

    <SummaryBody entry={entry} />
</div>

<style>
</style>