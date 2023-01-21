<script lang="ts">
    import AssignedIssues from "./lib/AssignedIssues.svelte";
    import CreatedIssues from "./lib/CreatedIssues.svelte";
    import MentionedIssues from "./lib/MentionedIssues.svelte";
    import AssignedPRs from "./lib/AssignedPRs.svelte";
    import CreatedPRs from "./lib/CreatedPRs.svelte";
    import MentionedPRs from "./lib/MentionedPRs.svelte";
    import ReviewRequests from "./lib/ReviewRequests.svelte";
    const organisation = "localstack";
</script>

<main
    class="container mx-auto p-8 flex h-screen justify-between overflow-scroll"
>
    <CreatedIssues {organisation} />
    <AssignedIssues {organisation} />
    <MentionedIssues {organisation} />
    <CreatedPRs {organisation} />
    <AssignedPRs {organisation} />
    <MentionedPRs {organisation} />
    <ReviewRequests {organisation} />
</main>
