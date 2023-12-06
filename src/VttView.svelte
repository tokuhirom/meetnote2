<script lang="ts">
  import {onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";

  let filename = "";
  let logs : {start_time: string, end_time: string, text: string}[] = [];

  onMount(async () => {
    console.log("onMount...")
    filename = location.search.replace(/^\?filename=/, '').replace(/\.md$/, '.vtt');
    logs = await invoke("load_webvtt", {filename: filename});
  });
</script>

<main class="container">
  <h2>{filename} - log</h2>

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