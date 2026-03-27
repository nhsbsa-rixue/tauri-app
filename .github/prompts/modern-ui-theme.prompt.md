---
description: "Apply modern UI theme to the page using Skeleton UI cerberus theme and Tailwind CSS"
agent: "agent"
---

Apply a modern UI theme to the +page.svelte layout page and its child components. Follow the existing page layout structure.

## Context

- Stack: Svelte 5, Skeleton UI v3 (cerberus theme), Tailwind CSS v4, Lucide icons
- The cerberus theme is already configured in [app.html](../../src/app.html) and [app.css](../../src/app.css)
- No new dependencies should be added

## Phase 1: Page Shell — [+page.svelte](../../src/routes/+page.svelte)

1. **Header** — Replace `bg-red-500` with Skeleton surface styling (`preset-filled-surface-500` or similar). Add an app title using Skeleton typography, subtle border/shadow for depth.
2. **Main grid** — Keep `grid-cols-1 md:grid-cols-[2fr_4fr_1fr]`. Remove `bg-green-500` from center column; apply neutral background or transparent.
3. **Left sidebar (Items)** — Apply Skeleton surface card/panel styling with subtle visual separation.
4. **Right sidebar** — Replace `bg-yellow-500` with a styled aside panel matching the design language.
5. **Footer** — Replace `bg-blue-500` with a Skeleton-themed muted footer.
6. Add consistent `gap`, `padding`, and subtle borders between sections.

## Phase 2: Items Component — [items.svelte](../../src/lib/items.svelte)

7. Restyle heading with Skeleton typography classes.
8. Style item list using Skeleton card/list styling; improve empty state.
9. Add remove button per item using Lucide `X` icon (`removeItem` already in [storeItems.ts](../../src/stores/storeItems.ts)).
10. Add "Clear All" button using Skeleton `btn` preset (`clearItems` already in [storeItems.ts](../../src/stores/storeItems.ts)).

## Phase 3: Dishes Component — [dishes.svelte](../../src/lib/dishes.svelte)

11. Enhance dish cards — add hover effects, shadows, better spacing between emoji and label.
12. Improve filter chips grid spacing/layout.
13. Pagination already uses Skeleton — likely no changes needed.

## Constraints

- Keep existing grid ratios (`2fr 4fr 1fr`)
- Use Skeleton UI presets exclusively (no custom colors) for cerberus theme consistency
- Scope: styling only — no new routes, backend changes, or new dependencies
- No placeholder colors (red/green/yellow/blue-500) should remain

## Verification

1. Run `npm run dev` and visually inspect the page
2. Confirm no placeholder colors remain
3. Confirm responsive behavior: single column on mobile, 3-column on md+
4. Confirm filter chips, dish cards, pagination render correctly with cerberus theme
5. Confirm items sidebar interaction (add/remove) works
