<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    export let file = {
        filename: string,
        content: string
    };

    let editMode = false;
    let editingContent : string | null = null;

    async function deleteItem(filename: string) {
        console.log(`Delete file: ${filename}`)
        await invoke("delete_file", {filename});
        return true;
    }

    async function enterEditingMode() {
        editMode = true
        editingContent = file.content
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
</script>

<div class="file">
    <div>
        <h2 style="float: left;">{file.filename}</h2>
        <div style="float: right">
            <button on:click|preventDefault={enterEditingMode}>Edit</button>
            <button on:click={e => deleteItem(file.filename)}>Delete</button>
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
</style>