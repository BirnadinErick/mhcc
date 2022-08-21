<script lang="ts">
	let ks = [1,2,3,4,5,6,7,8]
</script>

<svelte:head>
	<title>Overview | MHCC</title>
</svelte:head>

<div class="my-4">
	<h2 class="py-4 text-2xl font-serif">Overview.</h2>
</div>

<div class="w-auto overflow-clip grid grid-cols-3 grid-rows-3 gap-4 select-none cursor-default">

{#each ks as k}
	<div class="bg-white text-indigo p-4 shadow-md hover:shadow-xl transition-shadow ease-in-out text-center">
		<p class="mt-1 font-serif text-xl tracking-wide">Expiring Stocks</p>
		<p class="mt-1 font-mono text-4xl">1000</p>
		<p class="mt-1 font-sans text-sm uppercase">items</p>
	</div>
{/each}

</div>
