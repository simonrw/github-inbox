<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";

    export let organisation: string;

    let data = [];

    onMount(async () => {
        data = await invoke("fetch_created_prs", { organisation });
    });
</script>

<div class="w-[200px]">
    <h2 class="mx-auto text-center">Created PRs</h2>
    <ul>
        {#each data as item}
            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>
