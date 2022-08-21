CREATE TABLE sales_item
(
    sales_id INTEGER NOT NULL,
    stock_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    uprice   REAL    NOT NULL,
    FOREIGN KEY (sales_id)
        REFERENCES sales (sales_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    FOREIGN KEY (stock_id)
        REFERENCES stocks (stock_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    PRIMARY KEY (sales_id, stock_id)
);