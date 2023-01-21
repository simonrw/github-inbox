<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";

    export let organisation: string;

    let data = [];

    onMount(async () => {
        data = await invoke("fetch_created_issues", { organisation });
    });
</script>

<div class="border w-[200px]">
    <h2 class="mx-auto text-center">Created issues</h2>
    <ul>
        {#each data as item}
            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>
