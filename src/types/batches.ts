export type RawBatch = {
    batch_number: number;
    batch_year: number;
    batch_month: number;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: number;
    pureness_score: number;
    total_pureness_score: number;
    outflow_total_pureness_score: number;
    outflow_total_weight: number;
    usage: string;
    batch_status: number
    deleted_at: number | null;
    origin: string | null;
};

export type BatchDB = {
    batch_number: number;
    batch_year: number;
    batch_month: number;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: number;
    pureness_score: number;
    total_pureness_score: number;
    batch_status: number;
    deleted_at: number | null;
    origin: string | null;
};

export type BatchOutflowDB = {
    batch_number: number;
    batch_year: number;
    sack_amount: number;
    total_weight: number;
    total_pureness_score: number;
    usage: string;
};

export type DataTableBatch = {
    key: string;
    batch_number: number;
    batch_year: number;
    expire_date: string;
    seed: string;
    coating: string;
    brand: string;
    sack_weight: number;
    sack_amount: number;
    total_weight: string;
    pureness_score: string;
    total_pureness_score: string;
    batch_status: number;
    deleted_at: number | null;
    _searchIndex: string;
};

export type DataTableBatchOutflow = {
    batch_number: number;
    batch_year: number;
    sack_amount: number;
    total_weight: string;
    total_pureness_score: string;
    pureness_score: string;
    usage: string;
    _searchIndex: string;
};