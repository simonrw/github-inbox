<script lang="ts">
    export let raw;
</script>

<a
    class="bg-slate-800 rounded-lg px-6 py-8 shadow-xl font-normal m-4 block text-white hover:text-slate-200"
    href={raw.html_url}
    target="_blank"
    rel="noreferrer"
>
    {raw.title}
</a>

<style>
    a {
        font-family: "Noto Sans";
    }
</style>
