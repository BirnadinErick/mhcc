<script lang="ts">
	import type SidebarItemType from '../types/SidebarItemType';
	import { page } from "$app/stores";

	export let item: SidebarItemType;
</script>

<style>
	.active {
		@apply text-emerald-200 bg-emerald-900 shadow-lg
	}
</style>

<a href={item.loc}>
	<div
		class="my-4 hover:bg-emerald-700 text-emerald-900 hover:text-emerald-100  p-2 rounded-full hover:shadow-xl shadow-none transition-all duration-200 {$page.url.pathname === item.loc ? 'active' : ''}"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-6 w-6"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
			stroke-width="2"
		>
			<path stroke-linecap="round" stroke-linejoin="round" d={item.svg_val} />
		</svg>
	</div>
</a>
