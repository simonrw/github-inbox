<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let name = "";
    let greetMsg = "";
    let issues = "";

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        issues = await invoke("fetch_assigned_issues");
        console.log(issues);
    }
</script>

<div>
    <div class="row">
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={name}
        />
        <button on:click={greet}> Greet </button>
    </div>
    <p>{greetMsg}</p>
</div>

