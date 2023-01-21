<script lang="ts">
    import AssignedIssues from "./lib/AssignedIssues.svelte";
    import CreatedIssues from "./lib/CreatedIssues.svelte";
    import MentionedIssues from "./lib/MentionedIssues.svelte";
    import ReviewRequests from "./lib/ReviewRequests.svelte";
    const organisation = "localstack";
</script>

<main
    class="container mx-auto p-8 flex h-screen justify-between overflow-scroll"
>
    <CreatedIssues {organisation} />
    <AssignedIssues {organisation} />
    <MentionedIssues {organisation} />

    <ReviewRequests {organisation} />
</main>
