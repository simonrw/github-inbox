<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

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
                <a href={item.html_url} target="_blank" rel="noreferrer"
                    >{item.title}</a
                >
            </li>
        {/each}
    </ul>
</div>
