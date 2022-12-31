INSERT INTO staffs (staff_id, name, uname, passwd, role, date_enrolled)
VALUES (1, "Birnadin Erick", "bern", "1923040923urhskxml", 1, unixepoch());

INSERT INTO stocks (name, dispenser, quantity, uprice, date_expiry, staff_stocked)
VALUES ("panadol", "cocoa pharmacy", 100, 20.50, unixepoch() + 1000, 1);

INSERT INTO grn (date_returned, staff_returned)
VALUES (unixepoch() + 2003, 1);

INSERT INTO visitor (name, address, tpno, dob, nic)
VALUES ("Birnadin Erick", "117, David Road, Jaffna", "765812511", unixepoch() - 2003 * 10, "200314010404");

INSERT INTO sales (v_id, date_sold, staff_id)
VALUES (1, unixepoch(), 1);

INSERT INTO sales_item (sales_id, stock_id, quantity, uprice)
VALUES (1, 1, 10, (SELECT uprice FROM stocks WHERE stocks.stock_id = 1));