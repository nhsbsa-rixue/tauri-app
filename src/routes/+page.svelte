<script lang="ts">
  import { onMount } from "svelte";
  import Dishes from "../lib/dishes.svelte";
  import Items from "../lib/items.svelte";

  onMount(async () => {});
</script>

<div class="flex flex-col min-h-screen w-full h-full">
  <!-- Header -->
  <header class="bg-red-500 p-4">(header)</header>
  <!-- Main Content -->
  <main class="grid grid-cols-1 md:grid-cols-[2fr_4fr_1fr] flex-1">
    <!-- Sidebar (Left) -->
    <Items></Items>
    <div class="bg-green-500 p-4 space-y-4">
      <Dishes></Dishes>
    </div>
    <!-- Sidebar (Right) -->
    <aside class="bg-yellow-500 p-4">(sidebar)</aside>
  </main>
  <!-- Footer -->
  <footer class="bg-blue-500 p-4 mt-auto">(footer)</footer>
</div>
