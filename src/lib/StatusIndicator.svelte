<script lang="ts">
    import type {Entry} from "./entry";
    import {onMount} from "svelte";

    export let entry: Entry;
    let micWavAvailable: boolean = false;
    let mp3Available: boolean = false;
    let vttAvailable: boolean = false;
    let mdAvailable: boolean = false;

    onMount(async () => {
        await watchEntry();
    });

    $: if (entry) {
        watchEntry()
    }

    async function watchEntry() {
        micWavAvailable = await entry.hasMicWav();
        mp3Available = await entry.hasMp3();
        vttAvailable = await entry.hasVTT();
        mdAvailable = await entry.hasMD();
    }
</script>

<div>
    <div class="indicator">
        <span class="micWav" class:available={micWavAvailable} class:unavailable={!micWavAvailable}>WAV</span>
        <span class="mp3" class:available={mp3Available} class:unavailable={!mp3Available}>MP3</span>
        <span class="md" class:available={mdAvailable} class:unavailable={!mdAvailable}>Summary</span>
        <span class="vtt" class:available={vttAvailable} class:unavailable={!vttAvailable}>VTT</span>
    </div>
</div>

<style>
    .indicator span {
        margin: 4px;
    }

    .available {
        color: #4CAF50;
    }
    .unavailable {
        color: #9E9E9E;
    }
</style>