<script lang="ts">
    export let raw;
    console.log(raw);

    let logoColour: string;
    if (raw.draft) {
        logoColour = "text-slate-400";
    } else {
        logoColour = "text-green-500";
    }
    const logoClass = `inline mr-2 fill-current ${logoColour}`;
</script>

<div
    class="border-2 border-slate-800 rounded-lg p-4 mx-8 my-4 bg-slate-800 shadow-xl hover:border-slate-700"
>
    <a
        href={raw.html_url}
        target="_blank"
        rel="noreferrer"
        class="justify-between"
    >
        <div>
            {#if raw.pull_request}
                <svg
                    class={logoClass}
                    viewBox="0 0 16 16"
                    version="1.1"
                    width="16"
                    height="16"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M2.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.25 1a2.25 2.25 0 00-.75 4.372v5.256a2.251 2.251 0 101.5 0V5.372A2.25 2.25 0 003.25 1zm0 11a.75.75 0 100 1.5.75.75 0 000-1.5zm9.5 3a2.25 2.25 0 100-4.5 2.25 2.25 0 000 4.5zm0-3a.75.75 0 100 1.5.75.75 0 000-1.5z"
                    />
                    <path
                        d="M14 7.5a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0zm0-4.25a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0z"
                    />
                </svg>
            {:else}
                <svg
                    class="inline mr-2 fill-current text-green-500"
                    viewBox="0 0 16 16"
                    version="1.1"
                    width="16"
                    height="16"
                    aria-hidden="true"
                >
                    <path d="M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z" />
                    <path
                        fill-rule="evenodd"
                        d="M8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z"
                    />
                </svg>
            {/if}
            {raw.title}
        </div>
        <div class="text-slate-600">
            {raw.repo.owner}/{raw.repo.name}
        </div>
    </a>
</div>
