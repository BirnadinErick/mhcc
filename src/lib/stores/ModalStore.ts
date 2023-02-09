import { writable } from 'svelte/store';

const DeleteModalState = writable(false);
const DeleteItemState = writable(0);

export { DeleteModalState, DeleteItemState };
