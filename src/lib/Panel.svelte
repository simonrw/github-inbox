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

<div>
    <h2>{queryArgs.title}</h2>
    <ul>
        {#each data as item}
            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>

<style>
    div {
        min-width: 250px;
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
