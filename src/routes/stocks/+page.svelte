<script lang="ts">
	import { Render, Subscribe, createTable, createRender, DataBodyRow } from 'svelte-headless-table';
	import { addSortBy, addColumnOrder } from 'svelte-headless-table/plugins';
	import { readable } from 'svelte/store';
	import EditableTableCell from '$lib/components/EditableTableCell.svelte';
	import AddNewItemStore from "$lib/stores/AddNewItemStore";
	import dummy_data from './data';
import AddNewItemOverlay from '$lib/components/AddNewItemOverlay.svelte';

	const data = readable(dummy_data);

	const table = createTable(data, {
		sort: addSortBy({ disableMultiSort: true }),
		colOrder: addColumnOrder()
	});

	const updateData = (rowDataId: string, columnId: string, newValue: any) => {
		// In this case, the dataId of each item is its index in $data.
		// You can also handle any server-synchronization necessary here.
		const idx = parseInt(rowDataId);
		const currentItem = $data[idx];
		const key = columnId;
		const newItem = { ...currentItem, [key]: newValue };
		$data[idx] = newItem;
		$data = $data;

		// Sync with backend
	};

	// @ts-ignore
	const EditableCellLabel /*: DataLabel<Sample>*/ = ({ column, row, value }) => {
		return createRender(EditableTableCell, {
			row,
			column,
			value,
			onUpdateValue: updateData
		});
	};

	const columns = table.createColumns([
		table.column({
			header: 'ID',
			accessor: 'id',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'First Name',
			accessor: 'first_name',
			cell: EditableCellLabel,
			plugins: {
				sort: { invert: true }
			}
		}),
		table.column({
			header: 'Last Name',
			accessor: 'last_name',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Email',
			accessor: 'email',
			cell: EditableCellLabel
		})
	]);

	const { headerRows, rows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);
</script>

<svelte:head>
	<title>Stocks | MHCC</title>
</svelte:head>

<div class="flex justify-between my-2 lg:mx-8 mx-2">
	<h2 class="py-1 text-3xl font-serif">Stocks.</h2>
	<button
		class="p-3 mx-2 flex items-center space-x-1 bg-white text-indigo shadow-none hover:shadow-lg hover:shadow-emerald-200 hover:scale-105  transition-all duration-300 ease-out" on:click={()=>{AddNewItemStore.set(true)}}
	>
		<span class="p-1">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-6 h-6"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M13.5 16.875h3.375m0 0h3.375m-3.375 0V13.5m0 3.375v3.375M6 10.5h2.25a2.25 2.25 0 002.25-2.25V6a2.25 2.25 0 00-2.25-2.25H6A2.25 2.25 0 003.75 6v2.25A2.25 2.25 0 006 10.5zm0 9.75h2.25A2.25 2.25 0 0010.5 18v-2.25a2.25 2.25 0 00-2.25-2.25H6a2.25 2.25 0 00-2.25 2.25V18A2.25 2.25 0 006 20.25zm9.75-9.75H18a2.25 2.25 0 002.25-2.25V6A2.25 2.25 0 0018 3.75h-2.25A2.25 2.25 0 0013.5 6v2.25a2.25 2.25 0 002.25 2.25z"
				/>
			</svg>
		</span>
		<p class="px-2">Add New Item</p>
	</button>
</div>

<div class="h-full overflow-hidden my-4 lg:mx-8 mx-2">
	<div class="overflow-y-auto h-full">
		<table {...$tableAttrs} class="table-auto w-full  border-black select-none">
			<thead class="">
				{#each $headerRows as headerRow (headerRow.id)}
					<Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
						<tr {...rowAttrs} class="bg-emerald-200">
							{#each headerRow.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
									<th
										{...attrs}
										class="py-3 px-6 border-black font-thin text-left  font-serif text-xl tracking-wider"
										on:click={props.sort.toggle}
									>
										<Render of={cell.render()} />
										{#if props.sort.order === 'asc'}
											⬇️
										{:else if props.sort.order === 'desc'}
											⬆️
										{/if}
									</th>
								</Subscribe>
							{/each}
						</tr>
					</Subscribe>
				{/each}
			</thead>
			<tbody {...$tableBodyAttrs}>
				{#each $rows as row (row.id)}
					<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
						<tr {...rowAttrs} class="bg-white hover:bg-emerald-100">
							{#each row.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs>
									<td {...attrs} class="border-b-[1px] border-gray-300 py-3 text-left px-6">
										<Render of={cell.render()} />
									</td>
								</Subscribe>
							{/each}
						</tr>
					</Subscribe>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<AddNewItemOverlay >
	<form action="" class="">
		<label for="tl" class="text-white">telephone</label>
		<input type="text" name="telephone" id="tl">
	</form>
</AddNewItemOverlay>