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

<main
    class="container font-semibold mx-auto p-8 flex h-screen justify-between overflow-scroll"
>
    {#each roles as role}
        <Panel {organisation} {role} />
    {/each}
</main>
