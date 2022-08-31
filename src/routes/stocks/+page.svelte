<script lang="ts">
	// @ts-ignore
	import { Datatable } from 'svelte-simple-datatables';
	import data from './data';

	const settings = { columnFilter: true, css:false };
	let rows: any;
</script>

<svelte:head>
	<title>Stocks | MHCC</title>
</svelte:head>

<div class="flex justify-between my-2">
	<h2 class="py-1 text-3xl font-serif">Stocks.</h2>
	<button class="px-2 py-1 mx-2 flex items-center space-x-2 bg-white text-indigo shadow-lg">
		<span class=" p-1">➕</span>
		<p>Create Item</p>
	</button>
</div>

<div class="h-full overflow-hidden bg-slate-700">
	<Datatable {settings} {data} bind:dataRows={rows}>
		<thead>
			<th data-key="first_name">First Name</th>
			<th data-key="last_name">Last Name</th>
			<th data-key="email">Email</th>
		</thead>
		<tbody>
			{#if rows}
				{#each $rows as row}
					<tr>
						<td>{row.first_name}</td>
						<td>{row.last_name}</td>
						<td>{row.email}</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</Datatable>
</div>