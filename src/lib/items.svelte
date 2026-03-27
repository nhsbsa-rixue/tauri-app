<script lang="ts">
  import { items, removeItem, clearItems } from "../stores";
  import { X, Trash2, ShoppingCart, Printer } from "@lucide/svelte";

  const total = $derived($items.reduce((sum, item) => sum + item.price, 0));

  const mergedItems = $derived(() => {
    const map = new Map<
      string,
      { name_en: string; price: number; qty: number }
    >();
    for (const item of $items) {
      const existing = map.get(item.name_en);
      if (existing) {
        existing.qty += 1;
      } else {
        map.set(item.name_en, {
          name_en: item.name_en,
          price: item.price,
          qty: 1,
        });
      }
    }
    return Array.from(map.values());
  });

  function printReceipt() {
    const merged = mergedItems();
    const now = new Date().toLocaleString();

    const rows = merged
      .map(
        (m) => `<tr>
          <td style="text-align:left;padding:2px 0;">${m.name_en}</td>
          <td style="text-align:center;padding:2px 4px;">x${m.qty}</td>
          <td style="text-align:right;padding:2px 0;">$${(m.price * m.qty).toFixed(2)}</td>
        </tr>`,
      )
      .join("");

    const html = `<!DOCTYPE html>
<html><head><title>Receipt</title>
<style>
  @page { size: 80mm auto; margin: 0; }
  body { font-family: monospace; width: 72mm; margin: 4mm; font-size: 12px; color: #000; }
  h2 { text-align: center; margin: 0 0 4px; font-size: 14px; }
  .date { text-align: center; font-size: 10px; margin-bottom: 8px; }
  table { width: 100%; border-collapse: collapse; }
  .divider { border-top: 1px dashed #000; margin: 6px 0; }
  .total { font-weight: bold; font-size: 14px; display: flex; justify-content: space-between; }
  .footer { text-align: center; font-size: 10px; margin-top: 8px; }
</style></head><body>
  <h2>Order Receipt</h2>
  <div class="date">${now}</div>
  <div class="divider"></div>
  <table>
    <thead><tr>
      <th style="text-align:left;">Item</th>
      <th style="text-align:center;">Qty</th>
      <th style="text-align:right;">Price</th>
    </tr></thead>
    <tbody>${rows}</tbody>
  </table>
  <div class="divider"></div>
  <div class="total"><span>Total</span><span>$${total.toFixed(2)}</span></div>
  <div class="footer">Thank you!</div>
</body></html>`;

    const printWindow = window.open("", "_blank", "width=320,height=600");
    if (printWindow) {
      printWindow.document.write(html);
      printWindow.document.close();
      printWindow.focus();
      printWindow.print();
      printWindow.close();
    }
  }
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

  <!-- Total -->
  {#if $items.length > 0}
    <div class="pt-3 mt-3 border-t border-surface-200-800 space-y-3">
      <div class="flex justify-between font-bold text-lg">
        <span>Total</span>
        <span>${total.toFixed(2)}</span>
      </div>
      <button
        type="button"
        class="btn preset-filled-primary-500 w-full"
        onclick={() => printReceipt()}
      >
        <Printer size="16" />
        <span>Print</span>
      </button>
    </div>
  {/if}
</div>
