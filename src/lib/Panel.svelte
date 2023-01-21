<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Item from "./Item.svelte";

    export let title: string;
    export let organisation: string;

    let data = [];

    onMount(async () => {
        data = await invoke("fetch_assigned_issues", { organisation });
    });
</script>

<div>
    <h2>{title}</h2>
    <ul>
        {#each data as item}

            <li>
                <Item raw={item} />
            </li>
        {/each}
    </ul>
</div>
