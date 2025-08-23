<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Switch } from "@skeletonlabs/skeleton-svelte";
  import { Pagination } from "@skeletonlabs/skeleton-svelte";
  import { addItem } from "../stores";


  let types = $state<Types[]>([]);
  let dishes = $state<Dishes[]>([]);

  let filter: String = $state("");

  const dishOnFilter =  $derived((type: String) => {
    if (type) {
      const dishesAfterFilter = dishes.filter((dish: Dishes) => dish.menu_type === type);
      return dishesAfterFilter;
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

  const clickOnFilter = (type: String) => {
    page = 1;
    if (filter === type) {
      filter = "";
    } else {
      filter = type;
    }
  }





</script>

<div class="container">

    <div class="switch-container">
    {#each types as type}
      <Switch 
        name={`icons-${type.id}`}
        checked={filter === type.name_en}
        onCheckedChange={() => clickOnFilter(type.name_en)}
        compact
      >
        {#snippet inactiveChild()}
          <span>{type.name_cn}</span>
        {/snippet}
        {#snippet activeChild()}
          <span>{type.name_cn}</span>
        {/snippet}
      </Switch>
    {/each}
  </div>

  <!-- Dish Grid -->
  <div class="grid">
    {#each dishOnPage(dishOnFilter(filter)) as dish}
      <div class="tile">
        <div style="font-size:2rem">{dish.img}</div>
        <button type="button" class="btn preset-filled-primary-500" onclick={() => addItem(dish)} >{dish.name_en}</button>
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


</div>
