-- Initial Migration for Creating Default Data
INSERT INTO game_server (id, created_at, updated_at, region_code, display_name) VALUES ("development", 173817452843, 173817452843, "US/Eastern", "Development");
INSERT INTO world (id, created_at, updated_at, game_server_id, display_name) VALUES ("comet", 173813803857, 173813803857, "development", "Comet");
INSERT INTO world (id, created_at, updated_at, game_server_id, display_name) VALUES ("crystal", 173813803857, 173813803857, "development", "Crystal");

INSERT INTO user (id, updated_at, username, role) VALUES (2916111731091046399, 173813803857, "ProjectCometDev", 3);

INSERT INTO item (id, updated_at, name, stack_size, is_unique, item_type, tradability, data, icon_asset, drop_model_asset) VALUES (2916111731090980863, 173813803857, "Gold", 999999999, 0, 0, 2, NULL, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
