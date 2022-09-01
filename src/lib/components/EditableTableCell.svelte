<script lang="ts">
	import type { BodyRow, DataColumn } from 'svelte-headless-table';

	type Item = $$Generic;

	export let row: BodyRow<Item>;
	export let column: DataColumn<Item>;
	export let value: any;
	export let onUpdateValue: Function;

	let isEditing = false;

	let inputElement: HTMLInputElement;
	$: if (isEditing) {
		inputElement?.focus();
	}

	const handleCancel = () => {
		isEditing = false;
	};
	const handleSubmit = () => {
		isEditing = false;
		if (row.isData()) {
			onUpdateValue(row.dataId, column.id, value);
		}
	};
</script>

<div>
	{#if !isEditing}
		<span on:click={() => (isEditing = true)}>
			{value}
		</span>
	{:else}
		<form on:submit|preventDefault={handleSubmit}>
			<input bind:this={inputElement} type="text" bind:value />
			<button type="submit">✅</button>
			<button on:click={handleCancel}>❌</button>
		</form>
	{/if}
</div>

<style>
	form {
		display: flex;
		gap: 0.5rem;
	}

	button {
		padding: 0;
		border: none;
		background: transparent;
	}
</style>
