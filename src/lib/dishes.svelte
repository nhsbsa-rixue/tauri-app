<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Check } from "@lucide/svelte";

  import { Pagination } from "@skeletonlabs/skeleton-svelte";
  import { addItem } from "../stores";

  let types = $state<Types[]>([]);
  let dishes = $state<Dishes[]>([]);

  let filter: String = $state("");

  const dishOnFilter = $derived((type: String) => {
    if (type) {
      const dishesAfterFilter = dishes.filter(
        (dish: Dishes) => dish.menu_type === type,
      );
      return dishesAfterFilter;
    }
    return dishes;
  });

  onMount(async () => {
    types = await invoke("list_types");
    dishes = await invoke("list_dishes");
  });

  let page = $state(1);
  const pageSize = 15;
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
  };
</script>

<div class="w-full h-full">
  <div class="flex flex-wrap gap-2 mb-4 justify-center">
    {#each types as type}
      <button
        onclick={() => clickOnFilter(type.name_en)}
        type="button"
        class="chip {filter === type.name_en
          ? 'preset-filled-primary-500'
          : 'preset-filled'} transition-all duration-150"
      >
        <span>{type.name_cn}</span>
        <span style="display: inline-block; width: 20px; text-align: center;">
          {#if filter === type.name_en}
            <Check size="16" />
          {/if}
        </span>
      </button>

      <!-- <Switch
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
      </Switch> -->
    {/each}
  </div>

  <!-- Dish Grid -->
  <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
    {#each dishOnPage(dishOnFilter(filter)) as dish}
      <button
        type="button"
        class="card preset-filled-surface-200-800 hover:preset-filled-primary-500 w-full p-3 flex flex-col items-center justify-center gap-2 rounded-xl shadow-sm hover:shadow-md hover:scale-[1.02] transition-all duration-150 cursor-pointer h-[140px]"
        onclick={() => addItem(dish)}
      >
        <span class="text-3xl">{dish.img}</span>
        <span class="text-sm font-medium text-center leading-tight"
          >{dish.name_en}</span
        >
      </button>
    {/each}
  </div>

  <Pagination
    data={dishOnFilter(filter)}
    classes="w-full flex justify-center"
    {page}
    {pageSize}
    onPageChange={(e) => (page = e.page)}
    alternative
  />
</div>
