<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";
    import { queryMapping } from "./utils";

    export let role: string;
    export let organisation: string;

    let data = [];
    const queryArgs = queryMapping(role);

    onMount(async () => {
        data = await invoke(queryArgs.query, { organisation });
    });
</script>

<div class="w-[200px]">
    <h2 class="text-zinc-300 mx-auto text-center">{queryArgs.title}</h2>
    <ul>
        {#each data as item}
            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>

<style>
    h2 {
        font-family: "Noto Sans";
    }
</style>
