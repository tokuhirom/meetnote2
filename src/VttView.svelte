<script lang="ts">
  import {afterUpdate, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import type {Entry} from "./lib/entry";
  import type {Caption} from "./lib/webvtt";

  export let entry:  Entry;
  let mp3 : string | undefined = undefined;
  let logs : Caption[] = [];

  let prevFilename = "";
  onMount(async () => {
    console.log("onMount...")
    await watchFile();
  });

  $: if (entry) {
    watchFile()
  }

  async function watchFile() {
    logs = await entry.read_vtt();
    mp3 = await entry.readMp3AsDataUri();
  }

   function seek(log: Caption) {
    const startSeconds = log.parseStartTimeMillis() / 1000;

    const audio = document.getElementsByTagName("audio")[0];
    audio.currentTime = startSeconds;
    audio.play();
  }
</script>

<main class="container">
  <audio controls>
    <source src="{mp3}">
    Your browser does not support the audio tag.
  </audio>

  <table>

  {#each logs as log}
    <tr>
      <td><button on:click|preventDefault={() => seek(log)}>[{log.startTime.replace(/\.\d{3}$/, '')}]</button></td>
      <td>{log.text}</td>
    </tr>
  {/each}
  </table>
</main>

<style>
  td {
    vertical-align: top;
  }
  button {
    padding: 4px;
  }
</style>
