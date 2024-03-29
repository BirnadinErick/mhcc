<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import { Render, Subscribe, createTable } from 'svelte-headless-table';
	import { addSortBy, addColumnOrder } from 'svelte-headless-table/plugins';

	import AddNewItemOverlay from '$lib/components/AddNewItemOverlay.svelte';
	import AddNewItemButton from '$lib/components/AddNewItemButton.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	import { onMount } from 'svelte';
	import EditableCellLabelGenerator from '$lib/table/EditableCellLabel';
	import type GetPatient from '$lib/models/PatientModels';
	import hasScrolledHalfWay from '$lib/utils/indicators';

	var searchParam: string;
	let searchElement: HTMLInputElement;

	let tableContainerElement: HTMLDivElement;
	var scrollTimes: number = 0;
	var lastAction: string = '';

	// insert form

	let patient_name: string;
	let dob: Date;
	let address: string;
	let tpno: string;
	let nic: string;

	async function insert() {
		const new_patient = {
			patient_name: patient_name,
			tpno: tpno,
			nic: nic,
			address: address,
			dob: dob
		};

		await invoke('insert_patient', { newPatient: new_patient });
	}

	// get data to populate
	let data: Writable<Array<GetPatient>> = writable([]);
	let data_temp: Array<GetPatient>; // to hold original data during search

	async function updateData(rowDataId: string, columnId: string, newValue: any) {
		// In this case, the dataId of each item is its index in $data.
		// You can also handle any server-synchronization necessary here.
		const idx = parseInt(rowDataId);
		const currentItem = $data[idx];

		let newItem: GetPatient;
		newItem = { ...currentItem };

		switch (columnId) {
			case 'tpno':
			case 'address':
			case 'patient_name':
				newItem[columnId] = newValue;
				break;
			default:
				break;
		}

		$data[idx] = newItem;

		// Sync with backend
		let update_status = await invoke('update_patient', {
			newPatient: newItem
		});

		console.debug(update_status);
	}
	const EditableCellLabel = EditableCellLabelGenerator(updateData);

	async function search(term: string) {
		// get backup
		if (lastAction !== 's') {
			data_temp = $data;
		}

		// invoke api
		let patients: Array<GetPatient> = await invoke('search_patients', { query: term });

		// set the data
		data.set(patients);
	}

	async function populateTable(scrollOffset: number) {
		let patients: Array<GetPatient> = await invoke('get_patients', { offSet: scrollOffset });

		patients.forEach((_, idx) => {
			patients[idx].address = patients[idx].address.slice(0, 30) + '...';
		});

		// to be valid when called due to user-scroll
		data.set([...$data, ...patients]);
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
		table.column({
			header: 'ID',
			accessor: 'patient_id'
		}),
		table.column({
			header: 'Patient',
			accessor: 'patient_name',
			cell: EditableCellLabel,
			plugins: {
				sort: { invert: true }
			}
		}),
		table.column({
			header: 'Address',
			accessor: 'address',
			cell: EditableCellLabel
		}),
		table.column({
			header: 'NIC',
			accessor: 'nic',
			plugins: {
				sort: { invert: true }
			}
		}),
		table.column({
			header: 'DOB',
			accessor: 'dob'
		}),
		table.column({
			header: 'Telephone',
			accessor: 'tpno',
			cell: EditableCellLabel
		})
	]);

	const { headerRows, rows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);

	document.addEventListener('keydown', keyMapper);
</script>

<svelte:head>
	<title>Patients | MHCC</title>
</svelte:head>

<div class="flex justify-between my-2 lg:mx-8 mx-2">
	<PageHeader headerTitle="Patients." />
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

<AddNewItemOverlay addCallback={insert}>
	<div class="">
		<label for="patient_name" class="text-white">Name</label>
		<input
			class="block p-1"
			type="text"
			bind:value={patient_name}
			name="patient_name"
			id="patient_name"
		/>
	</div>

	<div class="">
		<label for="address" class="text-white">Address</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="number"
			name="address"
			id="address"
			bind:value={address}
		/>
	</div>

	<div class="">
		<label for="nic" class="text-white ">NIC</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="number"
			name="nic"
			id="nic"
			bind:value={nic}
		/>
	</div>

	<div class="">
		<label for="dob" class="text-white">DOB</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="date"
			name="dob"
			id="dob"
			bind:value={dob}
		/>
	</div>

	<div class="">
		<label for="tpno" class="text-white ">Telephone</label>
		<input
			class="block p-1 mt-1 focus:outline-indigo"
			type="text"
			name="tpno"
			id="tpno"
			bind:value={tpno}
		/>
	</div>
</AddNewItemOverlay>
