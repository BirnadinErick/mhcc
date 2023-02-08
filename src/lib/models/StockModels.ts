import type GetType from './BaseModels';

type StocksGet =
	| {
			stock_id: number;
			stock_name: string;
			uprice: number;
			quantity: number;
			dispensers_name: string;
			date_expiry: Date;
	  }
	| GetType;

export default StocksGet;
