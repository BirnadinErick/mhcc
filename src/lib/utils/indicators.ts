function hasScrolledHalfWay(tableContainerElement: HTMLDivElement): boolean {
	return (
		Math.abs(
			tableContainerElement.scrollHeight -
				tableContainerElement.clientHeight -
				tableContainerElement.scrollTop
		) < 4
	);
}

export default hasScrolledHalfWay;
