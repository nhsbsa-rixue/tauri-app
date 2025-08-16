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

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
