<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";

    export let organisation: string;

    let data = [];

    onMount(async () => {
        data = await invoke("fetch_mentioned_issues", { organisation });
    });
</script>

<div class="w-[200px]">
    <h2>Mentioned issues</h2>
    <ul>
        {#each data as item}
            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>
