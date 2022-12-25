<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import { Render, Subscribe, createTable, createRender } from 'svelte-headless-table';
	import { addSortBy, addColumnOrder } from 'svelte-headless-table/plugins';

	import AddNewItemOverlay from '$lib/components/AddNewItemOverlay.svelte';
	import AddNewItemButton from '$lib/components/AddNewItemButton.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import EditableTableCell from '$lib/components/EditableTableCell.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	import { onMount } from 'svelte';

	var searchParam: string;
	let searchElement: HTMLInputElement;

	let tableContainerElement: HTMLDivElement;
	var scrollTimes: number = 0;
	var lastAction: string = '';

	// @ts-ignore
	const EditableCellLabel /*: DataLabel<Sample>*/ = ({ column, row, value }) => {
		return createRender(EditableTableCell, {
			row,
			column,
			value,
			onUpdateValue: updateData
		});
	};

	type StocksGet = {
		stock_id: number;
		stock_name: string;
		uprice: number;
		quantity: number;
		dispensers_name: string;
		date_expiry: Date;
	};
	// var stocks: Array<StocksGet>;

	// get data to populate
	let data: Writable<Array<StocksGet>> = writable([]);
	let data_temp: Array<StocksGet>; // to hold original data during search

	function updateData(rowDataId: string, columnId: string, newValue: any) {
		// In this case, the dataId of each item is its index in $data.
		// You can also handle any server-synchronization necessary here.
		const idx = parseInt(rowDataId);
		const currentItem = $data[idx];
		const key = columnId;
		const newItem = { ...currentItem, [key]: newValue };
		$data[idx] = newItem;
		$data = $data;

		// Sync with backend
	}

	async function search(term: string) {
		// get backup
		if (lastAction !== 's') {
			data_temp = $data;
		}

		// invoke api
		let stocks: Array<StocksGet> = await invoke('search_stocks', { term: term });

		// set the data
		data.set(stocks);
	}

	async function populateTable(scrollOffset: number) {
		let stocks: Array<StocksGet> = await invoke('get_stocks', { offSet: scrollOffset });

		stocks.forEach((element, idx) => {
			// when deserilized, float precision is lost, thus extra decimal values are added
			//  with random value
			//  e.g. in PostgreSQL and Rust = 442.23 --> in JSVm = 442.23569874512685432154
			stocks[idx].uprice = parseFloat(element.uprice.toFixed(2));
		});

		// to be valid when called due to user-scroll
		data.set([...$data, ...stocks]);
	}

	function undoLastAction() {
		switch (lastAction) {
			case 's':
				data.set(data_temp);
				data_temp = [];
				searchParam = '';
				lastAction = '';
				break;
			default:
				break;
		}
	}

	function keyMapper(event: KeyboardEvent) {
		if (event.isComposing) {
			return;
		}

		if (event.ctrlKey === true && event.key === 's') {
			searchElement.focus();
			return;
		} else if (event.key === 'Enter') {
			if (document.activeElement === searchElement) {
				search(searchParam.toLowerCase());
				lastAction = 's';
			}
			return;
		} else if (event.key === 'Escape') {
			undoLastAction();
		} else {
			return;
		}
	}

	function hasScrolledHalfWay(): boolean {
		return (
			Math.abs(
				tableContainerElement.scrollHeight -
					tableContainerElement.clientHeight -
					tableContainerElement.scrollTop
			) < 4
		);
	}

	function tableInfScroll() {
		// TODO: discard prevoius data when scrolled to somewhat down
		//  e.g. if row count exceeds 300, append new 100 and discard old 100 or 200
		//  PERFORMANCE GAIN and less memory footprint

		if (lastAction === 's') {
			return;
		}

		if (hasScrolledHalfWay()) {
			scrollTimes++;
			populateTable(scrollTimes);
		}
	}

	onMount(async () => {
		await populateTable(scrollTimes);

		setTimeout(() => {
			tableContainerElement.addEventListener('scroll', tableInfScroll, {
				passive: true
			});
		}, 3000);
	});

	const table = createTable(data, {
		sort: addSortBy({ disableMultiSort: true }),
		colOrder: addColumnOrder()
	});

	const columns = table.createColumns([
		table.column({
			header: 'ID',
			accessor: 'stock_id',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Stock Name',
			accessor: 'stock_name',
			cell: EditableCellLabel,
			plugins: {
				sort: { invert: true }
			}
		}),
		table.column({
			header: 'Unit Price',
			accessor: 'uprice',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Quantity',
			accessor: 'quantity',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Expirying Date',
			accessor: 'date_expiry',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Dispenser',
			accessor: 'dispensers_name',
			cell: EditableCellLabel
		})
	]);

	const { headerRows, rows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);

	document.addEventListener('keydown', keyMapper);
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
				placeholder="Search (CTRL + s, ENTER)"
				class="p-4"
			/>
		</div>
		<AddNewItemButton />
	</div>
</div>

<div class="h-full overflow-hidden my-4 lg:mx-8 mx-2">
	<div class="overflow-y-auto h-full" id="table-id" bind:this={tableContainerElement}>
		<table {...$tableAttrs} class="table-auto w-full  border-black select-none">
			<thead class="">
				{#each $headerRows as headerRow (headerRow.id)}
					<Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
						<tr {...rowAttrs} class="bg-emerald-200">
							{#each headerRow.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
									<th
										{...attrs}
										class="py-3 px-6 border-black font-thin text-left  font-serif text-lg tracking-wider"
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
