<script lang="ts">
  import { Pagination } from '@skeletonlabs/skeleton-svelte';

  const { dishes = [] } = $props<{ dishes: Array<Dishes> }>();

  let selectedItems: Dishes[] = [];



  function selectDish(dish: Dishes) {
    selectedItems = [...selectedItems, dish];
  }

  let page = $state(1);
  const pageSize = 8;
  const dishOnPage = $derived((s: Dishes[]) => s.slice((page - 1) * pageSize, page * pageSize));
</script>

<div class="container">

    <!-- Selected Items -->
  <div class="selected">
    <h2>Selected Items</h2>
    {#if selectedItems.length === 0}
      <p>No items selected</p>
    {:else}
      <ul>
        {#each selectedItems as item, i}
          <li>{i + 1}. {item.name_en}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <!-- Dish Grid -->
  <div class="grid">
    {#each  dishOnPage(dishes) as dish}
      <div class="tile"
  >
    <div style="font-size:2rem">{dish.img}</div>
    <div>{dish.name_en}</div>
  </div>
    {/each}
  </div>

  <Pagination data={dishes} {page} pageSize={pageSize} onPageChange={(e) => (page = e.page)}
 alternative/>

</div>
<!-- 
<style>
  .container {
    display: flex;
    gap: 2rem;
    padding: 2rem;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    flex: 2;
  }
  .tile {
    border: 2px solid #ccc;
    border-radius: 1rem;
    padding: 1rem;
    text-align: center;
    cursor: pointer;
    font-size: 1.5rem;
    background: #fff;
    transition: 0.2s;
  }
  .tile:hover {
    background: #f0f0f0;
  }
  .selected {
    flex: 1;
    border: 2px solid #ccc;
    border-radius: 1rem;
    padding: 1rem;
    background: #fafafa;
  }
  .selected h2 {
    margin-bottom: 1rem;
  }
</style> -->
