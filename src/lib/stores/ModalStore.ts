import { writable } from 'svelte/store';

const DeleteModalState = writable(false);
const GRNModalState = writable(false);

const DeleteItemState = writable(0);
const GRNItemState = writable(0);

export { DeleteModalState, DeleteItemState, GRNModalState, GRNItemState };
