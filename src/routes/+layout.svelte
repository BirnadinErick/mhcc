<script lang="ts">
	import SidebarItem from '$lib/components/SidebarItem.svelte';
	import TransitionPage from '$lib/components/TransitionPage.svelte';
	import type SidebarItemType from '$lib/types/SidebarItemType';

	import { page } from '$app/stores';

	import '../css/app.css';

	let sidebar_items: Array<SidebarItemType> = [
		{
			tip: 'Home',
			svg_val:
				'M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2',
			loc: '/'
		},
		{
			tip: 'Stocks',
			svg_val: 'M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4',
			loc: '/stocks'
		},
		{
			tip: 'Patients',
			svg_val:
				'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z',
			loc: '/patients'
		},
		{
			tip: 'Instruments',
			svg_val:
				'M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z',
			loc: '/instruments'
		},
		{
			tip: 'Notifications',
			svg_val:
				'M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9',
			loc: '/notifications'
		},
		{
			tip: 'Settings',
			svg_val:
				'M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4',
			loc: '/settings'
		}
	];
</script>

<main class="h-screen bg-slate-200 flex w-full cursor-default">
	<nav
		class="bg-green w-16 py-8 px-2 flex flex-col justify-center items-center overflow-y-auto overflow-x-hidden flex-shrink-0"
	>
		{#each sidebar_items as item}
			<SidebarItem {item} />
		{/each}
	</nav>

	<TransitionPage url={$page.url}>
		<slot />
	</TransitionPage>
</main>
