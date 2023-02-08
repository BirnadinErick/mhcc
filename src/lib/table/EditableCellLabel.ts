/* eslint-disable @typescript-eslint/no-explicit-any */
import EditableTableCell from '$lib/table/EditableTableCell.svelte';
import { createRender } from 'svelte-headless-table';

function EditableCellLabelGenerator(updateData: CallableFunction) {
	const EditableCellLabel = ({ column, row, value }: { column: any; row: any; value: any }) => {
		return createRender(EditableTableCell, {
			row,
			column,
			value,
			onUpdateValue: updateData
		});
	};

	return EditableCellLabel;
}

export default EditableCellLabelGenerator;
