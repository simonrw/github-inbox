<script lang="ts">
    import Panel from "./lib/Panel.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    const roles: string[] = [
        "created-issues",
        "assigned-issues",
        "mentioned-issues",
        "created-prs",
        "assigned-prs",
        "mentioned-prs",
        "review-requests",
    ];

    let availableOrganisations = [];
    let organisation: string | undefined = undefined;

    onMount(async () => {
        availableOrganisations = await invoke("fetch_orgs");
        availableOrganisations.push("simonrw");
    });
</script>

<main>
    {#if organisation}
        {#each roles as role}
            <Panel {organisation} {role} />
        {/each}
    {:else}
        <!-- organisation chooser -->
        <div>
            <label
                for="organisations"
                class="block mb-2 text-sm font-medium text-gray-900 text-white"
                >Select an organisation</label
            >
            <select
                id="organisations"
                bind:value={organisation}
                class="bg-gray-700 border-gray-600 text-white rounded-lg text-sm w-full"
            >
                {#each availableOrganisations as org}
                    <option>{org}</option>
                {/each}
            </select>
        </div>
    {/if}
</main>

<style>
    main {
        width: 100%;
        display: flex;
        overflow: scroll;
        padding: 2rem;
        margin-left: auto;
        margin-right: auto;
        font-weight: 600;
        justify-content: space-between;
        height: 100vh;
    }
</style>
