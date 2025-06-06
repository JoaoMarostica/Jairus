export type BalanceDB = {
    sack_amount: number;
    total_weight: number;
    total_pureness_score: number;
};

export type DataTableBalanceOutflow = {
    key: string;
    sack_amount: number;
    total_weight: string;
    total_pureness_score: string;
    _searchIndex: string;
};