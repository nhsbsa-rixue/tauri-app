<script lang="ts">
  import { UtensilsCrossed } from "@lucide/svelte";
  import Dishes from "../lib/dishes.svelte";
  import Items from "../lib/items.svelte";
</script>

<div class="app-shell">
  <header class="global-nav typography-nav-link">
    <div class="brand">
      <UtensilsCrossed size="16" />
      <h1>Menu Order</h1>
    </div>
    <span>Dining Room</span>
  </header>

  <main class="app-main-grid">
    <section class="panel-parchment">
      <Items />
    </section>

    <section class="panel-main">
      <Dishes />
    </section>

    <aside class="panel-right"></aside>
  </main>

  <footer class="app-footer typography-fine-print">
    <div>Tauri + SvelteKit App</div>
    <div>Action Blue powered ordering experience</div>
  </footer>
</div>
