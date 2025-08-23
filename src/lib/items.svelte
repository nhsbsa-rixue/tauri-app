<script lang="ts">
import { items } from '../stores';

</script>



<div>
  <!-- Selected Items -->
  <div class="selected">
    <h2>Selected Items</h2>
    {#if $items.length === 0}
      <p>No items selected</p>
    {:else}
      <ul>
        {#each $items as item, i}
          <li>{i + 1}. {item.name_en}</li>
        {/each}
      </ul>
    {/if}
  </div>
</div>