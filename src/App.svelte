<script lang="ts">
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import NowRecordingIndicator from "./lib/NowRecordingIndicator.svelte";
  import VttView from "./VttView.svelte";
  import {DataRepo} from "./lib/data_repo";
  import type {Entry} from "./lib/entry";

  let selectedEntry: Entry | undefined = undefined;
  let entries: Entry[] = []

  let data_repo = new DataRepo();

  onMount(async () => {
    entries = await data_repo.list_entries();
  });

  // todo: better polling logic
  setInterval(async () => {
    entries = await data_repo.list_entries();
  }, 3000);

  function onSelectEntry(file: Entry) {
    selectedEntry = file;
  }
</script>

<main class="container">
  <NowRecordingIndicator />

  <div class="main-container">
    <div class="files">
      {#each entries as entry}
        <FileItem entry={entry} onSelectEntry={onSelectEntry}/>
      {/each}
    </div>
    <div class="vtt">
      {#if selectedEntry}
        <h2>{selectedEntry.title()}</h2>
        <pre class="summary">{selectedEntry.summary}</pre>
        <VttView entry={selectedEntry} />
      {/if}
    </div>
  </div>
</main>

<style>
  .main-container {
    display: flex;
  }
  .files {
    flex: 0 0 30%;
    overflow-y: auto;
  }
  .vtt {
    flex: 1;
  }

  .summary {
    margin-bottom: 10px;
    border-bottom: #396cd8 1px solid;
  }
</style>
