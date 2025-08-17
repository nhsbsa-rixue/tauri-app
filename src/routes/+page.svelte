<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Dishes from "../lib/dishes.svelte";

  let types  = $state<Array<any>>([]);
  let dishes = $state<Array<any>>([
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
  ]);

  onMount(async () => {
    types = await invoke("list_types");

    dishes = await invoke("list_dishes");
  });

  </script>


<main class="container">

  <h2>Dishes</h2>

  <Dishes dishes={dishes} ></Dishes>


</main>