<script lang="ts">
</script>

<div>
	<div class="flex items-center justify-start my-1 space-x-4">
		<button class="overlay-btn hover:bg-sky-200 ring-bg-sky-200">GRN</button>

		<button class="overlay-btn hover:bg-red ring-red" on:click={() => true}>Delete</button>
	</div>
</div>

<style>
	.overlay-btn {
		@apply p-2 ring-2 hover:text-indigo font-sans text-black transition-colors duration-200 ease-in-out text-sm tracking-wide;
	}
</style>
