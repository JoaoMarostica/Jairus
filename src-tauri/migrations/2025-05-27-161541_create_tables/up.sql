CREATE TABLE tb_outflow (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sack_amount INTEGER,
    total_weight INTEGER,
    total_pureness_score REAL,
    usage TEXT
);

CREATE TABLE tb_seed (
    scientific_name TEXT NOT NULL PRIMARY KEY,
    popular_name TEXT
);

CREATE TABLE tb_coating (
    coating_name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE tb_brand (
    brand_name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE tb_weight (
    sack_weight INTEGER NOT NULL PRIMARY KEY
);

CREATE TABLE tb_brand_weights (
    brand TEXT NOT NULL,
    sack_weight INTEGER NOT NULL,
    FOREIGN KEY (brand) REFERENCES tb_brand(brand_name),
    FOREIGN KEY (sack_weight) REFERENCES tb_weight(sack_weight),
    PRIMARY KEY (brand, sack_weight)
);

CREATE TABLE tb_batch (
    batch_number INTEGER NOT NULL,
    batch_year INTEGER NOT NULL,
    batch_month INTEGER,
    seed TEXT NOT NULL,
    coating TEXT NOT NULL,
    brand TEXT NOT NULL,
    sack_weight INTEGER,
    sack_amount INTEGER,
    total_weight INTEGER,
    pureness_score REAL,
    total_pureness_score REAL,
    origin TEXT,
    FOREIGN KEY (seed) REFERENCES tb_seed(scientific_name),
    FOREIGN KEY (coating) REFERENCES tb_coating(name),
    FOREIGN KEY (brand) REFERENCES tb_brand(name),
    FOREIGN KEY (sack_weight) REFERENCES tb_weight(sack_weight),
    PRIMARY KEY (batch_number, batch_year)
);