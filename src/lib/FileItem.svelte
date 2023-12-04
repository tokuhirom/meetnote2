<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    export let file = {
        filename: string,
        content: string
    };

    async function deleteItem(filename: string) {
        console.log(`Delete file: ${filename}`)
        await invoke("delete_file", {filename});
        return true;
    }
</script>

<div class="file">
    <div>
        <h2 style="float: left;">{file.filename}</h2>
        <div style="float: right">
            <button on:click={e => deleteItem(file.filename)}>Delete</button>
        </div>
    </div>
    <pre style="clear: both">{file.content}</pre>
</div>
