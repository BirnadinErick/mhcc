<script lang="ts">
	import { writable } from 'svelte/store';
	import { Render, Subscribe, createTable, createRender } from 'svelte-headless-table';
	import { addSortBy, addColumnOrder } from 'svelte-headless-table/plugins';

	import AddNewItemOverlay from '$lib/components/AddNewItemOverlay.svelte';
	import AddNewItemButton from '$lib/components/AddNewItemButton.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import EditableTableCell from '$lib/components/EditableTableCell.svelte';

	import dummy_data from './data';

	var searchParam: string;
	let searchElement: HTMLInputElement;

	// get data to populate
	const data = writable(dummy_data);

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

	function search() {
		data.set(
			dummy_data.filter((record) => {
				return (record.first_name + record.last_name + record.email)
					.toLowerCase()
					.includes(searchParam.toLowerCase());
			})
		);
	}

	document.addEventListener('keydown', (event) => {
		if (event.isComposing) {
			return;
		}
		if (event.ctrlKey === true && event.key.toLowerCase() === 's') {
			searchElement.focus();
			return;
		}
	});
</script>

<svelte:head>
	<title>Stocks | MHCC</title>
</svelte:head>

<div class="flex justify-between my-2 lg:mx-8 mx-2">
	<PageHeader />
	<div class="flex justify-between items-center space-x-4">
		<div class="flex items-center space-x-2">
			<input
				bind:this={searchElement}
				bind:value={searchParam}
				type="text"
				name="search"
				id="search"
				placeholder="Search 🔎"
				class="p-4"
				on:input={search}
			/>
		</div>
		<AddNewItemButton />
	</div>
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

<AddNewItemOverlay>
	<form action="" class="">
		<label for="tl" class="text-white">telephone</label>
		<input type="text" name="telephone" id="tl" />
	</form>
</AddNewItemOverlay>
