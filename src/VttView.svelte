<script lang="ts">
  import {afterUpdate, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";

  export let file:  {filename: string, content: string};
  let mp3 = "";
  let logs : {start_time: string, end_time: string, text: string}[] = [];

  let prevFilename = "";
  onMount(async () => {
    console.log("onMount...")
    await watchFile();
  });

  $: if (file) {
    watchFile()
  }

  async function watchFile() {
    logs = await invoke("load_webvtt", {filename: file.filename.replace(".md", ".mp3")});
    mp3 = await invoke("read_data_tag_mp3", {filename: file.filename.replace(".md", ".mp3")});
  }

  function convertToSeconds(time: string): number {
    const splitTime = time.split(':').map(Number);
    return splitTime[0] * 3600 + splitTime[1] * 60 + splitTime[2];
  }

   function seek(log:{start_time: string, end_time: string, text: string}) {
    const start = convertToSeconds(log.start_time);
    const audio = document.getElementsByTagName("audio")[0];
    audio.currentTime = start;
    audio.play();
  }
</script>

<main class="container">
  <h2>{file.filename} - log</h2>

  <audio controls>
    <source src="{mp3}">
    Your browser does not support the audio tag.
  </audio>

  <table>

  {#each logs as log}
    <tr>
      <td><a href="#" on:click|preventDefault={() => seek(log)}>[{log.start_time}]</a></td>
      <td>{log.text}</td>
    </tr>
  {/each}
  </table>
</main>

<style>
  td {
    vertical-align: top;
  }
</style>
