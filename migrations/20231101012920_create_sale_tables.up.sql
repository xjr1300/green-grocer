-- 野菜テーブル作成
CREATE TABLE IF NOT EXISTS vegetables (
    id UUID NOT NULL,
    name VARCHAR(80) NOT NULL,
    unit_price INTEGER NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id)
);
-- 販売テーブル作成
CREATE TABLE IF NOT EXISTS sales (
    id UUID NOT NULL,
    sold_at TIMESTAMP WITH TIME ZONE NOT NULL,
    total_price INTEGER NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id)
);
-- 販売明細テーブル作成
CREATE TABLE IF NOT EXISTS sale_details (
    id UUID NOT NULL,
    sale_id UUID NOT NULL,
    vegetable_id UUID NOT NULL,
    sold_unit_price INTEGER NOT NULL,
    sold_quantity INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (sale_id) REFERENCES sales (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (vegetable_id) REFERENCES vegetables (id) ON DELETE CASCADE ON UPDATE CASCADE
);
