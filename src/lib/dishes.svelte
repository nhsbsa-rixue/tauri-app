<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { addItem } from "../stores";

  let types = $state<Types[]>([]);
  let dishes = $state<Dishes[]>([]);

  let filter = $state("");
  let page = $state(1);
  const pageSize = 15;

  const filteredDishes = $derived(
    filter
      ? dishes.filter((dish: Dishes) => dish.menu_type === filter)
      : dishes,
  );

  const totalPages = $derived(
    Math.max(1, Math.ceil(filteredDishes.length / pageSize)),
  );

  const pagedDishes = $derived(
    filteredDishes.slice((page - 1) * pageSize, page * pageSize),
  );

  const pageNumbers = $derived(
    Array.from({ length: totalPages }, (_, i) => i + 1),
  );

  onMount(async () => {
    types = await invoke("list_types");
    dishes = await invoke("list_dishes");
  });

  $effect(() => {
    if (page > totalPages) {
      page = totalPages;
    }
  });

  const clickOnFilter = (type: string) => {
    page = 1;
    if (filter === type) {
      filter = "";
    } else {
      filter = type;
    }
  };

  const goToPage = (target: number) => {
    if (target >= 1 && target <= totalPages) {
      page = target;
    }
  };
</script>

<div class="w-full h-full">
  <div class="flex flex-wrap gap-2 mb-6 justify-center">
    {#each types as type}
      <button
        onclick={() => clickOnFilter(type.name_en)}
        type="button"
        class="btn-chip typography-caption {filter === type.name_en
          ? 'is-selected'
          : ''}"
      >
        <span>{type.name_cn}</span>
      </button>
    {/each}
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4">
    {#each pagedDishes as dish}
      <button type="button" class="card-dish" onclick={() => addItem(dish)}>
        <span class="text-4xl">{dish.img}</span>
        <span class="typography-body-strong text-center leading-tight"
          >{dish.name_en}</span
        >
        <span
          class="typography-caption"
          style="color: var(--color-ink-muted-80);"
          >${dish.price.toFixed(2)}</span
        >
      </button>
    {/each}
  </div>

  <nav
    class="mt-6 flex items-center justify-center gap-2"
    aria-label="Pagination"
  >
    <button
      type="button"
      class="page-btn"
      onclick={() => goToPage(page - 1)}
      disabled={page === 1}
    >
      Prev
    </button>
    {#each pageNumbers as currentPage}
      <button
        type="button"
        class="page-btn {currentPage === page ? 'is-current' : ''}"
        onclick={() => goToPage(currentPage)}
      >
        {currentPage}
      </button>
    {/each}
    <button
      type="button"
      class="page-btn"
      onclick={() => goToPage(page + 1)}
      disabled={page === totalPages}
    >
      Next
    </button>
  </nav>
</div>
