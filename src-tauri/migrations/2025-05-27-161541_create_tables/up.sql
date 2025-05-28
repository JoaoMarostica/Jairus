CREATE TABLE tb_outflow (
    outflow_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sack_amount INTEGER NOT NULL,
    total_weight INTEGER NOT NULL,
    total_pureness_score REAL NOT NULL,
    usage TEXT NOT NULL
);

CREATE TABLE tb_seed (
    scientific_name TEXT NOT NULL PRIMARY KEY,
    popular_name TEXT NOT NULL
);

CREATE TABLE tb_coating (
    coating_name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE tb_brand (
    brand_name TEXT NOT NULL,
    sack_weight INTEGER NOT NULL,
    PRIMARY KEY (brand_name, sack_weight)
);

CREATE TABLE tb_batch (
    batch_number INTEGER NOT NULL,
    batch_year INTEGER NOT NULL,
    batch_month INTEGER NOT NULL,
    seed TEXT NOT NULL,
    coating TEXT NOT NULL,
    brand TEXT NOT NULL,
    sack_weight INTEGER NOT NULL,
    sack_amount INTEGER NOT NULL,
    total_weight INTEGER NOT NULL,
    pureness_score REAL NOT NULL,
    total_pureness_score REAL NOT NULL,
    origin TEXT NOT NULL,
    FOREIGN KEY (seed) REFERENCES tb_seed(scientific_name),
    FOREIGN KEY (coating) REFERENCES tb_coating(name),
    FOREIGN KEY (brand) REFERENCES tb_brand(name),
    FOREIGN KEY (sack_weight) REFERENCES tb_weight(sack_weight),
    PRIMARY KEY (batch_number, batch_year)
);