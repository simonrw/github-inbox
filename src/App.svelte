<script lang="ts">
    import Panel from "./lib/Panel.svelte";
    const organisation = "localstack";
</script>

<main
    class="container mx-auto p-8 flex h-screen justify-between overflow-scroll"
>
    <!-- assigned issues -->
    <Panel {organisation} title="Assigned issues" />
    <Panel {organisation} title="Issues with Mentions" />
    <Panel {organisation} title="Created PRs" />
    <Panel {organisation} title="Review requests" />
</main>
