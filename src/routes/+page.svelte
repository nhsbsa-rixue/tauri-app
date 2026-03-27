<script lang="ts">
  import { onMount } from "svelte";
  import { UtensilsCrossed } from "@lucide/svelte";
  import Dishes from "../lib/dishes.svelte";
  import Items from "../lib/items.svelte";

  onMount(async () => {});
</script>

<div class="flex flex-col min-h-screen w-full h-full bg-surface-50-950">
  <!-- Header -->
  <header
    class="preset-filled-surface-200-800 px-6 py-4 flex items-center gap-3 border-b border-surface-200-800 shadow-sm"
  >
    <UtensilsCrossed size="24" />
    <h1 class="text-xl font-bold tracking-tight">Menu Order</h1>
  </header>
  <!-- Main Content -->
  <main
    class="grid grid-cols-1 md:grid-cols-[2fr_4fr_1fr] flex-1 divide-x divide-surface-200-800"
  >
    <!-- Sidebar (Left) -->
    <Items></Items>
    <div class="p-4 space-y-4 overflow-y-auto">
      <Dishes></Dishes>
    </div>
    <!-- Sidebar (Right) -->
    <aside class="p-4 preset-filled-surface-100-900"></aside>
  </main>
  <!-- Footer -->
  <footer
    class="preset-filled-surface-200-800 px-6 py-3 mt-auto border-t border-surface-200-800 text-xs opacity-60 text-center"
  >
    Tauri + SvelteKit App
  </footer>
</div>
