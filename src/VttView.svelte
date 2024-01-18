<script lang="ts">
  import {afterUpdate, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import type {Entry} from "./lib/entry";
  import type {Caption} from "./lib/webvtt";

  export let entry:  Entry;
  let mp3 : string | undefined = undefined;
  let logs : Caption[] = [];

  onMount(async () => {
    console.log("onMount...")
    await watchFile();
  });

  $: if (entry) {
    watchFile()
  }

  async function watchFile() {
    console.log("watchFile");
    try {
      logs = await entry.readVTT();
    } catch (e) {
      logs = [];
      console.error(`Cannot get VTT: ${e}`);
    }

    try {
      mp3 = await entry.readMp3AsDataUri();
      const audio = document.getElementsByTagName("audio")[0] as HTMLAudioElement;
      audio.load();
    } catch (e) {
      console.error(`MP3: ${e}`);
    }
  }

   function seek(log: Caption) {
    const startSeconds = log.parseStartTimeMillis() / 1000;

    const audio = document.getElementsByTagName("audio")[0] as HTMLAudioElement;
    audio.currentTime = startSeconds;
    audio.play();
  }
</script>

<main class="container">
  {#if mp3}
  <audio controls>
    <source src="{mp3}">
    Your browser does not support the audio tag.
  </audio>
  {/if}

  {#if logs}
    <table>
    {#each logs as log}
      <tr>
        <td><button on:click|preventDefault={() => seek(log)}>[{log.startTime.replace(/\.\d{3}$/, '')}]</button></td>
        <td>{log.text}</td>
      </tr>
    {/each}
    </table>
  {:else}
    VTT log is not available yet.
  {/if}
</main>

<style>
  td {
    vertical-align: top;
  }
  button {
    padding: 4px;
  }
</style>
