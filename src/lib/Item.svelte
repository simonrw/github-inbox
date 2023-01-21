<script lang="ts">
    export let raw;
</script>

<a href={raw.html_url} target="_blank" rel="noreferrer">
    {raw.title}
</a>

<style>
    a {
        font-family: "Noto Sans";
        background-color: #1e293b;
        display: block;
        padding-left: 1.5rem;
        padding-right: 1.5rem;
        padding-top: 2rem;
        padding-bottom: 2rem;
        margin: 1rem;
        color: #ffffff;
        font-weight: 400;
        border-radius: 0.5rem;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1),
            0 10px 10px -5px rgba(0, 0, 0, 0.04);
    }

    a:hover {
        color: #e2e8f0;
    }
</style>
