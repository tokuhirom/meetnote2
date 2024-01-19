<script lang="ts">
    export let isRecording: boolean;
</script>

<div>
    {#if isRecording}
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