import { writable } from 'svelte/store';

const OverlayState = writable(false);

export default OverlayState;
