<script lang="ts">
    import AssignedIssues from "./lib/AssignedIssues.svelte";
    const organisation = "localstack";
</script>

<main
    class="container mx-auto p-8 flex h-screen justify-between overflow-scroll"
>
    <AssignedIssues {organisation} />
</main>
