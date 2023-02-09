<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { GRNItemState, GRNModalState } from '$lib/stores/ModalStore';

	let quantity: string;

	async function save_grn() {
		const grn_status = await invoke('save_grn', {
			stockId: $GRNItemState,
			quantity: quantity,
			staffId: 1
		});

		if (grn_status) {
			$GRNModalState = false;
		}
	}
</script>

<div class="flex justify-between items-center mb-4">
	<h1 class="text-lg font-serif tracking-wider">Add GRN.</h1>
</div>

<form class="my-2">
	<!-- svelte-ignore a11y-autofocus -->
	<input
		class="text-indigo p-2 "
		type="number"
		placeholder="Quantity"
		autofocus
		bind:value={quantity}
	/>
</form>

<div class="flex items-center justify-start mt-8 mb-2 space-x-4">
	<button class="overlay-btn hover:bg-sky-200 ring-sky-200" on:click={save_grn}> Save. </button>
</div>

<style>
	.overlay-btn {
		@apply px-4 py-2 ring-2 hover:text-indigo font-serif text-white transition-colors duration-200 ease-in-out text-lg tracking-wider;
	}
</style>
