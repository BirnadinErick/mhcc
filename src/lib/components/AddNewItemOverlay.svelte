<script lang="ts">
	import AddNewItemStore from '$lib/stores/AddNewItemStore';
	import { cubicOut, cubicIn } from 'svelte/easing';
	import { fade, fly } from 'svelte/transition';
</script>

{#if $AddNewItemStore === true}
	<!-- overlay -->
	<div
		class="absolute bg-green/30 top-0 left-0 h-screen w-screen flex items-center justify-end text-white overflow-hidden"
		in:fade={{ duration: 300, easing: cubicOut }}
		out:fade={{ duration: 200, easing: cubicIn }}
		on:click={() => {
			AddNewItemStore.set(false);
		}}
	>
		<!-- popup -->
		<div
			class="bg-emerald-900 h-screen w-auto py-16 px-8"
			on:click|stopPropagation
			in:fly={{ x: 2000, duration: 300, easing: cubicOut }}
		>
			<!-- title -->
			<div class="flex justify-between items-center">
				<h1 class="text-3xl font-serif">Add New Stock.</h1>
			</div>
			<!-- form -->
			<div class="my-8 text-indigo">
				<slot />
			</div>
			<!-- buttons -->
			<div class="flex items-center justify-end w-full my-2 space-x-6">
				<button class="overlay-btn hover:bg-yellow ring-yellow" type="reset">Clear.</button>
				<button class="overlay-btn hover:bg-red ring-red" type="submit">Add.</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.overlay-btn {
		@apply px-4 py-2 ring-2 hover:text-indigo font-serif text-white transition-colors duration-200 ease-in-out text-lg tracking-wide;
	}
</style>
