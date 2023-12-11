<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {WebviewWindow} from "@tauri-apps/api/window";

    export let file: {
        filename: string | null,
        content: string | null
    } = {
        filename: null,
        content: null
    };

    let editMode = false;
    let editingContent : string | null = null;

    async function deleteItem() {
        console.log(`Delete file: ${file.filename}`)
        await invoke("delete_file", {filename: file.filename});
        return true;
    }

    async function enterEditingMode() {
        editMode = true
        editingContent = file.content
    }

    async function openLog() {
        console.log("open log");
        const webview = new WebviewWindow('vttlog', {
            url: "vtt.html?filename=" + file.filename
        });
        await webview.emit("filename", file.filename);
    }

    async function saveItem() {
        console.log(`Save file: ${file.filename}`)
        await invoke("save_file", {
            filename: file.filename,
            content: editingContent
        });
        file.content = editingContent
        editMode = false;
    }

    async function regenerateSummaryItem() {
        console.log(`Regenerate summary: ${file.filename}`)
        await invoke("regenerate_summary", {filename: file.filename});
        return true;
    }
</script>

<div class="file">
    <div>
        <h2 style="float: left;">{file.filename}</h2>
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
        <pre style="clear: both">{file.content}</pre>
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