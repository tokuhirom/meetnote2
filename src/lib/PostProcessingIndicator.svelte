<script lang="ts">
    import type {PostProcessStatus} from "./postprocess";

    export let status: PostProcessStatus;
</script>

<div class="container">
    <div class="basename">{status.basename}</div>
    <div class="message">{status.message}</div>
</div>

<style>
    .container {
        background-color: #396cd8;
        padding: 9px;
        margin-bottom: 8px;
    }
</style>

