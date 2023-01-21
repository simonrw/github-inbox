<script lang="ts">
    import Panel from "./lib/Panel.svelte";
    const organisation = "localstack";

    const roles: string[] = [
        "created-issues",
        "assigned-issues",
        "mentioned-issues",
        "created-prs",
        "assigned-prs",
        "mentioned-prs",
        "review-requests",
    ];
</script>

<main>
    {#each roles as role}
        <Panel {organisation} {role} />
    {/each}
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
