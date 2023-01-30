<script lang="ts">
  import Panel from "./lib/Panel.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Notifications from "./lib/Notifications.svelte";

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
  let username: string | undefined = undefined;

  let ready = false;

  onMount(async () => {
    availableOrganisations = await invoke("fetch_orgs");
  });

  function tryMoveOn() {
    if (username !== undefined && organisation !== undefined) {
      ready = true;
    }
  }
</script>

<main class="flex mx-auto overflow-scroll min-width-800 h-screen">
  {#if ready}
    <Notifications {username} />
    {#each roles as role}
      <Panel {username} {organisation} {role} />
    {/each}
  {:else}
    <!-- organisation chooser -->
    <div>
      <label
        for="username"
        class="block mb-2 text-sm font-medium text-gray-900 text-white"
        >What is your username?
      </label>
      <input
        class="rounded text-black"
        placeholder="Username"
        type="text"
        bind:value={username}
      />

      <label
        for="organisations"
        class="block mb-2 text-sm font-medium text-gray-900 text-white"
        >Select an organisation</label
      >
      <select
        id="organisations"
        bind:value={organisation}
        class="bg-gray-700 border-gray-600 text-black rounded-lg text-sm w-full"
      >
        {#each availableOrganisations as org}
          <option>{org}</option>
        {/each}
      </select>
      <button
        class="text-white px-4 py-2 bg-slate-700 rounded-lg shadow-xl hover:text-gray-200 mt-4"
        on:click={tryMoveOn}>Proceed</button
      >
    </div>
  {/if}
</main>
