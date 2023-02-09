<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import { Render, Subscribe, createTable, createRender } from 'svelte-headless-table';
	import { addSortBy, addColumnOrder } from 'svelte-headless-table/plugins';

	import AddNewItemOverlay from '$lib/components/AddNewItemOverlay.svelte';
	import AddNewItemButton from '$lib/components/AddNewItemButton.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	import { onMount } from 'svelte';
	import EditableCellLabelGenerator from '$lib/table/EditableCellLabel';
	import type StocksGet from '$lib/models/StockModels';
	import hasScrolledHalfWay from '$lib/utils/indicators';
	import ButtonsCell from '$lib/table/ButtonsCell.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import DeleteModalChild from '$lib/components/DeleteModalChild.svelte';
	import { DeleteItemState, DeleteModalState } from '$lib/stores/ModalStore';

	var searchParam: string;
	let searchElement: HTMLInputElement;

	let tableContainerElement: HTMLDivElement;
	var scrollTimes: number = 0;
	var lastAction: string = '';

	// insert form
	let stock_name: string;
	let dispensers_name: string;
	let quantity: number;
	let uprice: number;
	let date_expiry: string;

	async function insert_stock() {
		const new_stock = {
			stock_name: stock_name,
			uprice: uprice,
			quantity: quantity,
			date_expiry: date_expiry
		};

		await invoke('insert_stocks', { newStock: new_stock });
	}

	async function delete_stock(stock_id: number) {
		const delete_status = await invoke('delete_stock', { id: stock_id });
		if (delete_status) {
			$DeleteModalState = false;
		}
		document.location.reload();
	}

	function createTableActionsButtonCell({ row }: any, _: any) {
		return createRender(ButtonsCell, { stock_id: row.original.stock_id });
	}

	// get data to populate
	let data: Writable<Array<StocksGet>> = writable([]);
	let data_temp: Array<StocksGet>; // to hold original data during search

	async function updateData(rowDataId: string, columnId: string, newValue: any) {
		// In this case, the dataId of each item is its index in $data.
		// You can also handle any server-synchronization necessary here.
		const idx = parseInt(rowDataId);
		const currentItem = $data[idx];

		let newItem: StocksGet;
		newItem = { ...currentItem };

		switch (columnId) {
			case 'stock_name':
			case 'date_expiry':
				newItem[columnId] = newValue;
				break;
			case 'uprice':
				newItem[columnId] = parseFloat(newValue);
				break;
			case 'quantity':
				newItem[columnId] = parseInt(newValue);
				break;
			default:
				break;
		}

		$data[idx] = newItem;

		// Sync with backend
		let update_status = await invoke('update_stocks', {
			newStock: newItem
		});

		console.debug(update_status);
	}
	const EditableCellLabel = EditableCellLabelGenerator(updateData);
	// const ButtonsCell = ButtonCellGenerator(() => true);

	async function search(term: string) {
		// get backup
		if (lastAction !== 's') {
			data_temp = $data;
		}

		// invoke api
		let stocks: Array<StocksGet> = await invoke('search_stocks', { query: term });

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

			// truncate the dispenser name
			stocks[idx].dispensers_name =
				stocks[idx].dispensers_name.length > 20
					? stocks[idx].dispensers_name.slice(0, 19) + '...'
					: stocks[idx].dispensers_name;
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

	function tableInfScroll() {
		// TODO: discard prevoius data when scrolled to somewhat down
		//  e.g. if row count exceeds 300, append new 100 and discard old 100 or 200
		//  PERFORMANCE GAIN and less memory footprint

		if (lastAction === 's') {
			return;
		}

		if (hasScrolledHalfWay(tableContainerElement)) {
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
		// table.column({
		// 	header: 'ID',
		// 	accessor: 'stock_id'
		// }),
		table.column({
			header: 'Stock Name',
			accessor: 'stock_name',
			cell: EditableCellLabel,
			plugins: {
				sort: { invert: true }
			}
		}),
		table.column({
			header: 'Price',
			accessor: 'uprice',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Quantity',
			accessor: 'quantity',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Ex. Date',
			accessor: 'date_expiry',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'Dispenser',
			accessor: 'dispensers_name'
		}),
		table.display({
			header: 'Actions',
			cell: createTableActionsButtonCell,
			id: 'stock_id'
		})
	]);

	const { headerRows, rows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);

	document.addEventListener('keydown', keyMapper);
</script>

<svelte:head>
	<title>Stocks | MHCC</title>
</svelte:head>

<div class="flex justify-between my-2 lg:mx-8 mx-2">
	<PageHeader headerTitle="Stocks." />
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
								<Subscribe
									attrs={cell.attrs()}
									let:attrs
									props={cell.props()}
									let:props
								>
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
									<td
										{...attrs}
										class="border-b-[1px] border-gray-300 py-3 text-left px-6"
									>
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

<AddNewItemOverlay addCallback={insert_stock}>
	<div class="">
		<label for="stock_name" class="text-white">Name</label>
		<input
			class="block p-1"
			type="text"
			bind:value={stock_name}
			name="stock_name"
			id="stock_name"
		/>
	</div>

	<div class="">
		<label for="uprice" class="text-white"> Unit Price</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="number"
			name="uprice"
			id="uprice"
			bind:value={uprice}
		/>
	</div>

	<div class="">
		<label for="quantity" class="text-white ">Quantity</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="number"
			name="quantity"
			id="quantity"
			bind:value={quantity}
		/>
	</div>

	<div class="">
		<label for="date_expiry" class="text-white"> Expiry Date</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="date"
			name="date_expiry"
			id="date_expiry"
			bind:value={date_expiry}
		/>
	</div>

	<div class="">
		<label for="dispensers_name" class="text-white ">Dispenser</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="text"
			name="dispensers_name"
			id="dispensers_name"
			bind:value={dispensers_name}
		/>
	</div>
</AddNewItemOverlay>

<Modal>
	<DeleteModalChild item="stock" callback={() => delete_stock($DeleteItemState)} />
</Modal>
