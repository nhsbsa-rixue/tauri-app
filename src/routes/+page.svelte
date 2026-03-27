<script lang="ts">
  import { onMount } from "svelte";
  import { UtensilsCrossed } from "@lucide/svelte";
  import Dishes from "../lib/dishes.svelte";
  import Items from "../lib/items.svelte";
  import { items } from "../stores";

  const total = $derived($items.reduce((sum, item) => sum + item.price, 0));

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
    <aside class="p-4 preset-filled-surface-100-900 space-y-4">
      <h2 class="font-semibold text-sm uppercase tracking-wide opacity-60">
        Summary
      </h2>
      <div class="space-y-2 text-sm">
        <div class="flex justify-between">
          <span class="opacity-70">Items</span>
          <span class="font-medium">{$items.length}</span>
        </div>
        <hr class="hr" />
        <div class="flex justify-between font-bold text-lg">
          <span>Total</span>
          <span>${total.toFixed(2)}</span>
        </div>
      </div>
    </aside>
  </main>
  <!-- Footer -->
  <footer
    class="preset-filled-surface-200-800 px-6 py-3 mt-auto border-t border-surface-200-800 text-xs opacity-60 text-center"
  >
    Tauri + SvelteKit App
  </footer>
</div>
