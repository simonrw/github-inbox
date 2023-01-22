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

<div>
    <h2>{queryArgs.title}</h2>
    <ul>
        {#if data}
            {#each data as item}
                <li>
                    <Item raw={item} />
                </li>
            {/each}
        {:else}
            <li>Loading...</li>
        {/if}
    </ul>
</div>

<style>
    div {
        min-width: 250px;
    }

    li {
        color: #fff;
    }

    h2 {
        font-family: "Noto Sans";
        color: #d4d4d8;
        margin-left: auto;
        margin-right: auto;
        text-align: center;
        font-size: 1.125rem;
        line-height: 1.75rem;
    }
</style>
