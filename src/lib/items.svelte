<script lang="ts">
  import { items, removeItem, clearItems } from "../stores";
  import { X, Trash2, ShoppingCart } from "@lucide/svelte";
</script>

<div class="p-4 preset-filled-surface-100-900 h-full flex flex-col">
  <!-- Header -->
  <div class="flex items-center justify-between mb-4">
    <h2
      class="font-semibold text-sm uppercase tracking-wide opacity-60 flex items-center gap-2"
    >
      <ShoppingCart size="16" />
      Order ({$items.length})
    </h2>
    {#if $items.length > 0}
      <button
        type="button"
        class="btn preset-tonal-error btn-sm"
        onclick={() => clearItems()}
      >
        <Trash2 size="14" />
        <span>Clear</span>
      </button>
    {/if}
  </div>

  <!-- Item List -->
  <div class="flex-1 overflow-y-auto space-y-1">
    {#if $items.length === 0}
      <div
        class="flex flex-col items-center justify-center h-full opacity-40 gap-2"
      >
        <ShoppingCart size="32" />
        <p class="text-sm">No items selected</p>
      </div>
    {:else}
      {#each $items as item, i}
        <div
          class="flex items-center justify-between px-3 py-2 rounded-lg preset-filled-surface-200-800 group"
        >
          <span class="text-sm truncate flex-1">
            <span class="opacity-50 mr-1">{i + 1}.</span>
            {item.name_en}
          </span>
          <button
            type="button"
            class="btn-icon btn-icon-sm preset-tonal-error opacity-0 group-hover:opacity-100 transition-opacity"
            onclick={() => removeItem(item.itemId)}
          >
            <X size="14" />
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>
