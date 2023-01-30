<script lang="ts">
  export let username: string;

  let data = undefined;
</script>

<div class="min-w-[16rem] flex-initial w-[24rem]">
  <h2 class="text-white mx-auto text-center text-lg">Notifications</h2>
  <ul>
    {#if data}
      {#each data as item}
        <li class="text-white">
          <!-- <Item raw={item} /> -->
        </li>
      {/each}
    {:else}
      <li>Loading...</li>
    {/if}
  </ul>
</div>
