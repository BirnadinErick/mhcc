function search(param: string, collections: Array<string>) {
	return collections.filter((item) => {
		return item.toLowerCase().includes(param.toLowerCase());
	});
}

export default search;
