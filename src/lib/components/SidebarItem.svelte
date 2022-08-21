<script lang="ts">
	import type SidebarItemType from '../types/SidebarItemType';

	export let item: SidebarItemType;
</script>

<a href={item.loc}>
	<div
		class="my-4 hover:bg-emerald-900 text-emerald-900 hover:text-emerald-200  p-2 rounded-full hover:shadow-xl shadow-none transition-all duration-200"
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
