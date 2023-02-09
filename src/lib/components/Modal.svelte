<script lang="ts">
	// import { DeleteModalState } from '$lib/stores/ModalStore';
	import { cubicOut, cubicIn } from 'svelte/easing';
	import type { Writable } from 'svelte/store';
	import { fade } from 'svelte/transition';

	export let trigger: Writable<boolean>;
</script>

{#if $trigger === true}
	<!-- overlay -->
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="absolute bg-green/30 top-0 left-0 h-screen w-screen flex items-center justify-center text-white overflow-hidden"
		in:fade={{ duration: 300, easing: cubicOut }}
		out:fade={{ duration: 200, easing: cubicIn }}
		on:click={() => {
			trigger.set(false);
		}}
	>
		<!-- popup -->
		<div
			class="bg-emerald-900 py-16 px-16"
			on:click|stopPropagation
			in:fade={{ duration: 100, easing: cubicOut }}
		>
			<!-- title -->
			<slot />
		</div>
	</div>
{/if}
