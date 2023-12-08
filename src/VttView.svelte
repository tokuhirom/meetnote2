<script lang="ts">
  import {onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";

  let filename = "";
  let mp3 = "";
  let logs : {start_time: string, end_time: string, text: string}[] = [];


  onMount(async () => {
    console.log("onMount...")
    filename = location.search.replace(/^\?filename=/, '').replace(/\.md$/, '.vtt');
    logs = await invoke("load_webvtt", {filename: filename});
    mp3 = await invoke("read_data_tag_mp3", {filename: filename.replace(".vtt", ".mp3")});
  });
</script>

<main class="container">
  <h2>{filename} - log</h2>

  <audio controls>
    <source src="{mp3}">
    Your browser does not support the audio tag.
  </audio>

  <table>

  {#each logs as log}
    <tr>
      <td>{log.start_time}</td>
      <td>{log.end_time}</td>
      <td>{log.text}</td>
    </tr>
  {/each}
  </table>
</main>

<style>
</style>