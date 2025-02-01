-- Initial Migration for Creating Default Data
INSERT INTO game_server (id, created_at, updated_at, region_code, display_name) VALUES ("development", 173842930888, 173842930888, "US/Eastern", "Development");

INSERT INTO world (id, created_at, updated_at, game_server_id, display_name) VALUES ("clockwork", 173842930888, 173842930888, "development", "Clockwork");

INSERT INTO user (id, updated_at, username, role) VALUES (2916600401581113343, 173842930888, "CometAdmin", 4);

INSERT INTO item (id, updated_at, name, stack_size, is_unique, item_type, tradability, data, icon_asset, drop_model_asset) VALUES (2916600401581178879, 173842930888, "Gold", 999999999, 0, 0, 2, NULL, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
