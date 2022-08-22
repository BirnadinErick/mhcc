<script lang="ts">
	import SidebarItem from '$lib/components/SidebarItem.svelte';
	import type SidebarItemType from '$lib/types/SidebarItemType';

	import '../css/app.css';

	let sidebar_items: Array<SidebarItemType> = [
		{
			tip: 'Home',
			svg_val:
				'M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4',
			loc: '/'
		},
		{
			tip: 'Stocks',
			svg_val: 'M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4',
			loc: '/stocks'
		},
		{
			tip: 'Visitors',
			svg_val:
				'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z',
			loc: 'visitors'
		},
		{
			tip: 'Instruments',
			svg_val:
				'M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z',
			loc: '/instruments'
		},
		{
			tip: 'Settings',
			svg_val:
				'M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4',
			loc: '/settings'
		}
	];
</script>

<div class="h-screen w-screen overflow-hidden flex">
	<div class="bg-green w-16 py-8 px-2 flex flex-col justify-center items-center">
		{#each sidebar_items as item}
			<SidebarItem {item} />
		{/each}
	</div>
	<div class="flex-auto bg-gray-200 pt-16 pb-8 p-16 flex flex-col overflow-hidden h-full ">
		<div class="text-black  cursor-default select-none h-24">
			<h1 class="font-serif text-5xl">Good Morning</h1>
			<h3 class="text-medium mt-2 ml-1 capitalize text-gray-600">
				Welcome to Mathagal Health Care Center 🏥
			</h3>
		</div>

		<div class="flex-auto overflow-y-auto">
			<slot />
		</div>
	</div>
	<div class="bg-white w-72 px-8 py-16">
		<h2 class="text-2xl font-serif">Notifications</h2>
	</div>
</div>
