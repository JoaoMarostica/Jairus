export type SeedDB = {
    popular_name: string;
    scientific_name: string;
};

export type RawSeed = {
    id: number;
    name: string;   
    scientific_name: string;
    pmsPodium: number;
    pmsGolden: number;
    pmsConventional: number;
};

export type DataTableSeed = {
    key: string;
    popular_name: string;
    scientific_name: string;
    deleted_at: number | null;
    _searchIndex: string;
};
