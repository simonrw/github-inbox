<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";
    import { queryMapping } from "./utils";

    export let role: string;
    export let username: string;
    export let organisation: string;

    let data = undefined;
    const queryArgs = queryMapping(role);

    onMount(async () => {
        data = await invoke(queryArgs.query, { username, organisation });
    });
</script>

<div class="min-w-[16rem] flex-initial w-[24rem]">
    <h2 class="text-white mx-auto text-center text-lg">{queryArgs.title}</h2>
    <ul>
        {#if data}
            {#each data as item}
                <li class="text-white">
                    <Item raw={item} />
                </li>
            {/each}
        {:else}
            <li>Loading...</li>
        {/if}
    </ul>
</div>
