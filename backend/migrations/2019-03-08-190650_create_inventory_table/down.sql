DROP TRIGGER refresh_inventory_stock ON transaction_bundles;
DROP TRIGGER refresh_inventory_stock ON transaction_items;
DROP TRIGGER refresh_inventory_stock ON inventory;
DROP FUNCTION refresh_inventory_stock;
DROP MATERIALIZED VIEW inventory_stock;
DROP VIEW transactions_joined;
DROP TABLE inventory_bundle_items;
DROP TABLE inventory_bundles;
DROP TABLE transaction_items;
DROP TABLE transaction_bundles;
DROP TABLE transactions;
DROP TABLE inventory_tags;
DROP TABLE inventory;
DROP FUNCTION transaction_balance;
DROP TYPE INVENTORY_ITEM_CHANGE;