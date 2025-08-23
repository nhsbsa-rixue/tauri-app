<script lang="ts">
  import { onMount } from "svelte";
  import Dishes from "../lib/dishes.svelte";
  import Items from "../lib/items.svelte";

  onMount(async () => {
  });

  </script>




  <div class="grid h-screen grid-rows-[auto_1fr_auto]">
  <!-- Header -->
  <header class="bg-red-500 p-4">(header)</header>
  <!-- Grid Column -->
	<div class="grid grid-cols-1 md:grid-cols-[4fr_auto_1fr]">
		<!-- Sidebar (Left) -->
    <Items></Items>
		<!-- Main -->
		<main class="bg-green-500 p-4 space-y-4">
      <Dishes></Dishes>

		</main>
		<!-- Sidebar (Right) -->
		<aside class="bg-yellow-500 p-4">(sidebar)</aside>
	</div>
  <!-- Footer -->
  <footer class="bg-blue-500 p-4">(footer)</footer>
</div>

