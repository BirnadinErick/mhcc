<script lang="ts">
	import {
		GRNModalState,
		DeleteModalState,
		DeleteItemState,
		GRNItemState
	} from '$lib/stores/ModalStore';
	export let stock_id: number;

	function log(stockid: number) {
		console.log(stockid);
	}

	function show_delete_stock() {
		$DeleteModalState = true;
		$DeleteItemState = stock_id;
	}

	function show_grn() {
		$GRNModalState = true;
		$GRNItemState = stock_id;
	}
</script>

<div>
	<div class="flex items-center justify-start my-1 space-x-4">
		<button class="overlay-btn hover:bg-sky-200 ring-bg-sky-200" on:click={() => show_grn()}
			>GRN</button
		>

		<button class="overlay-btn hover:bg-red ring-red" on:click={() => show_delete_stock()}
			>Delete</button
		>
	</div>
</div>

<style>
	.overlay-btn {
		@apply p-2 ring-2 hover:text-indigo font-sans text-black transition-colors duration-200 ease-in-out text-sm tracking-wide;
	}
</style>
