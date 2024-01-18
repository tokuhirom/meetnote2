<script lang="ts">
    import type {Entry} from "./entry";
    import {onMount} from "svelte";

    export let entry: Entry;
    let vttAvailable: boolean = false;
    let mdAvailable: boolean = false;
    let mp3Available: boolean = false;

    onMount(async () => {
        await watchEntry();
    });

    $: if (entry) {
        watchEntry()
    }

    async function watchEntry() {
        vttAvailable = await entry.hasVTT();
        mdAvailable = await entry.hasMD();
        mp3Available = await entry.hasMp3();
    }
</script>

<div>
    <div class="indicator">
        <span class="md" class:available={mdAvailable} class:unavailable={!mdAvailable}>Summary</span>
        <span class="mp3" class:available={mp3Available} class:unavailable={!mp3Available}>MP3</span>
        <span class="vtt" class:available={vttAvailable} class:unavailable={!vttAvailable}>VTT</span>
    </div>
</div>

<style>
    .available {
        color: #4CAF50;
    }
    .unavailable {
        color: #9E9E9E;
    }
</style>