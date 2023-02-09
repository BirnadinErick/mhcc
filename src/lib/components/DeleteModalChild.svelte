<script lang="ts">
	export let item: string;
	export let callback: any;
</script>

<div class="flex justify-between items-center mb-6">
	<h1 class="text-lg font-serif tracking-wider">Delete this {item}?</h1>
</div>

<div class="flex items-center justify-center my-1 space-x-4">
	<button class="overlay-btn hover:bg-red ring-red" on:click={callback}> Yes, Delete. </button>
</div>

<style>
	.overlay-btn {
		@apply px-4 py-2 ring-2 hover:text-indigo font-serif text-white transition-colors duration-200 ease-in-out text-lg tracking-wider;
	}
</style>
