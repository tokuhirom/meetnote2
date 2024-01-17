<script>
    import {invoke} from "@tauri-apps/api/tauri";

    let now_recording = false;

    setInterval(async () => {
        now_recording = await invoke("is_recording");
    }, 1000);
</script>

<div>
    {#if now_recording}
        <div class="now-recording">
            Now recording...
        </div>
    {/if}
</div>

<style>
    .now-recording {
        background-color: brown;
        margin: 12px;
        border: crimson 5px solid;
    }
</style>