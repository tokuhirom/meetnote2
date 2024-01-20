<script lang="ts">
    import type {Entry} from "./entry";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";

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

    async function runPostProcess() {
        await invoke("start_postprocess", {dir: entry.path, command: "ALL"});
        entry = entry; // TODO needs refresh after post process... so, when is the best timing to do it?
    }
</script>

<div>
    <div class="indicator">
        <span class="micWav" class:available={micWavAvailable} class:unavailable={!micWavAvailable}>WAV</span>
        <span class="mp3" class:available={mp3Available} class:unavailable={!mp3Available}>MP3</span>
        <span class="md" class:available={mdAvailable} class:unavailable={!mdAvailable}>Summary</span>
        <span class="vtt" class:available={vttAvailable} class:unavailable={!vttAvailable}>VTT</span>
        {#if micWavAvailable}
            <button on:click={runPostProcess}>Run postprocess</button>
        {/if}
    </div>
</div>

<style>
    .indicator span {
        margin: 4px;
        background-color: #242424;
        padding: 3px;
    }

    .available {
        color: #4CAF50;
    }
    .unavailable {
        color: #9E9E9E;
    }
</style>