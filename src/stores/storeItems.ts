import { v4 as uuid } from "uuid";
import { writable } from 'svelte/store';

export const items = writable<Items[]>([]);

export const addItem = (dish: Dishes) => {
  items.update(currentItems => {
    const newItems = [...currentItems, { ...dish, itemId: uuid() } as Items];
    newItems.sort((a, b) => a.name_en.localeCompare(b.name_en));
    return newItems;
  });
}

export const removeItem = (itemId: string) => {
  items.update(currentItems => currentItems.filter(item => item.itemId !== itemId));
};

export const clearItems = () => {
  items.set([]);
};