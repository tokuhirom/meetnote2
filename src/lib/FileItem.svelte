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

<div class="file">
    <div>
        <h2 style="float: left;">{entry.title()}</h2>
        <div style="float: right" class="buttons">
            <button on:click|preventDefault={openLog}>Log</button>
        </div>
    </div>
    <pre style="clear: both">{entry.summary}</pre>
</div>

<style>
    pre {
        word-break: break-all;
    }
    .file .buttons {
        font-size: x-small;
    }
</style>