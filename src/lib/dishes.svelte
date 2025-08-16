<script lang="ts">
   export let dishes = [
    { id: 1, name_en: "Margherita", img: "🍕" },
    { id: 2, name_en: "Cheeseburger", img: "🍔" },
    { id: 3, name_en: "Fried Chicken", img: "🍗" },
    { id: 4, name_en: "Sushi", img: "🍣" },
    { id: 5, name_en: "Caesar Salad", img: "🥗" },
    { id: 6, name_en: "Pepperoni Pizza", img: "🍕" },
    { id: 7, name_en: "French Fries", img: "🍟" },
    { id: 8, name_en: "Pasta", img: "🍝" },
    { id: 9, name_en: "Taco", img: "🌮" },
    { id: 10, name_en: "Chicken Salad", img: "🥗" }
  ];

  let selectedItems: any[] = [];

  function selectDish(dish: any) {
    selectedItems = [...selectedItems, dish];
  }
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
    {#each dishes as dish}
      <div class="tile"
    on:click={() => selectDish(dish)}
  >
    <div style="font-size:2rem">{dish.img}</div>
    <div>{dish.name_en}</div>
  </div>
    {/each}
  </div>


</div>

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
</style>
