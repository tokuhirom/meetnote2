<script lang="ts">
  import {onMount} from "svelte";
  import FileItem from "./lib/FileItem.svelte";
  import NowRecordingIndicator from "./lib/NowRecordingIndicator.svelte";
  import VttView from "./VttView.svelte";
  import {DataRepo} from "./lib/data_repo";
  import type {Entry} from "./lib/entry";
  import SummaryView from "./lib/SummaryView.svelte";

  let selectedEntry: Entry | undefined = undefined;
  let entries: Entry[] = []

  let data_repo = new DataRepo();

  onMount(async () => {
    entries = await data_repo.list_entries();

    if (entries.length > 0) {
      selectedEntry = entries[0];
    }
  });

  // todo: better polling logic
  setInterval(async () => {
    entries = await data_repo.list_entries();
  }, 3000);

  function onSelectEntry(file: Entry) {
    selectedEntry = file;
  }

  async function onDelete() {
    entries = await data_repo.list_entries();

    if (entries.length > 0) {
      selectedEntry = entries[0];
    }
  }
</script>

<main class="container">
  <div class="main-container">
    <div class="files">
      <NowRecordingIndicator />
      {#each entries as entry}
        <FileItem entry={entry} onSelectEntry={onSelectEntry}/>
      {/each}
    </div>
    <div class="vtt">
      {#if selectedEntry}
        <h2>{selectedEntry.title()}</h2>
        <SummaryView entry="{selectedEntry}" onDelete={onDelete} />
        <hr class="separator" />
        <VttView entry={selectedEntry} />
      {/if}
    </div>
  </div>
</main>

<style>
  .main-container {
    display: flex;
    flex-direction: row;
    height: 100vh;
  }
  .files {
    flex: 0 0 30%;
    overflow-y: auto;
    padding-right: 9px;
    overflow-x: hidden;
    word-break: break-word;
    white-space: normal;
  }
  .vtt {
    flex: 1;
    overflow-y: auto;
    padding-left: 9px;
  }

  .separator {
    margin-bottom: 10px;
    /*border-bottom:#396cd8 1px solid;*/
  }
</style>
