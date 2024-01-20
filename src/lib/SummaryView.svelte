<script lang="ts">
    import SummaryBody from "./SummaryBody.svelte";
    import {Entry} from "./entry";
    import VttView from "../VttView.svelte";

    export let onDelete: () => void;

    export let entry: Entry;
    let pane = "summary";

    function showPane(p) {
        pane = p;
    }
</script>

<div>
    <h2>{entry.title()}</h2>

    <menu>
        <li><button class:selected="{pane === 'summary'}"
                    on:click={() => showPane("summary")}>Summary</button></li>
        <li><button class:selected="{pane === 'script'}"
                    on:click={() => showPane("script")}>Script</button></li>
    </menu>

    <div class="tab-content">
        {#if pane==="summary"}
            <SummaryBody entry={entry} onDelete={onDelete} />
        {:else if pane === "script"}
            <VttView entry={entry} />
        {:else}
            UNKNOWN PANE: {pane}
        {/if}
    </div>
</div>

<style>
    button {
        margin: 4px;
        width: 180px;
    }
    menu {
        display: flex;
        flex-direction: row;
        padding: 0;
        margin: 0;
    }
    menu li {
        list-style-type: none;
        margin-right: 2px;
        background-color: #a9a9a9;
    }
    menu button {
        margin: 0;
        border-radius: 2px 2px 0 0;
        border-style: none;
        border-top-width: 0;
    }
    menu button.selected {
        background-color: #0f0f2f;
    }
    .tab-content {
        background-color: #0f0f2f;
        padding: 8px;
        border-bottom-left-radius: 2px;
        border-bottom-right-radius: 2px;
    }
</style>