<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  import { Switch } from "@skeletonlabs/skeleton-svelte";
  import IconPin from "@lucide/svelte/icons/pin";

  import { Pagination } from "@skeletonlabs/skeleton-svelte";

  let types = $state<Array<Types>>([]);
  let dishes = $state<Array<Dishes>>([]);
  let dishSelected = $state<Array<Dishes>>([]);

  let filter: String = $state("");

  const dishOnFilter =  $derived((t: String) => {
    if (t) {
      return dishes.filter((dish: Dishes) => dish.menu_type === t);
    }
    return dishes;
  });

  onMount(async () => {
    types = await invoke("list_types");

    dishes = await invoke("list_dishes");

  });

  let page = $state(1);
  const pageSize = 8;
  const dishOnPage = $derived((s: Dishes[]) => {

    return s.slice((page - 1) * pageSize, page * pageSize);
  });

  function clickOnFilter(type: String) {
    if (filter === type) {
      filter = "";
    } else {
      filter = type;
    }
  }

</script>

<div class="container">
  <!-- Selected Items -->
  <div class="selected">
    <h2>Selected Items</h2>
    {#if dishSelected.length === 0}
      <p>No items selected</p>
    {:else}
      <ul>
        {#each dishSelected as item, i}
          <li>{i + 1}. {item.name_en}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <!-- Dish Grid -->
  <div class="grid">
    {#each dishOnPage(dishOnFilter(filter)) as dish}
      <div class="tile">
        <div style="font-size:2rem">{dish.img}</div>
        <div>{dish.name_en}</div>
      </div>
    {/each}
  </div>

  <Pagination
    data={dishOnFilter(filter)}
    {page}
    {pageSize}
    onPageChange={(e) => (page = e.page)}
    alternative
  />

  <hr />

  <div class="switch-container">
    {#each types as type}
      <Switch 
        name={`icons-${type.id}`}
        checked={filter === type.name_en}
        onCheckedChange={() => clickOnFilter(type.name_en)}
        compact
      >
        {#snippet inactiveChild()}
          <span>{type.name_en}</span>
          <IconPin size="14" style="opacity:0.3;" />
        {/snippet}
        {#snippet activeChild()}
          <span>{type.name_en}</span>
          <IconPin size="14" color="var(--your-accent-color, #f59e42)" />
        {/snippet}
      </Switch>
      <span></span>
    {/each}
  </div>
</div>
