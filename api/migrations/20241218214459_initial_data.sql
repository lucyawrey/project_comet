-- Initial Migration for Creating Default Data
INSERT INTO logical_server (id, created_at, updated_at, name) VALUES ("dev", 173817452843, 173817452843, "Development Server");
INSERT INTO world (id, updated_at, name, logical_server_id) VALUES (2916111731091177471, 173813803857, "Comet", "dev");
INSERT INTO world (id, updated_at, name, logical_server_id) VALUES (2916111731091111935, 173813803857, "Crystal", "dev");

INSERT INTO player (id, updated_at, username, role) VALUES (2916111731091046399, 173813803857, "ProjectCometDev", 3);

INSERT INTO item (id, updated_at, name, stack_size, is_unique, item_type, tradability, data, icon_asset, drop_model_asset) VALUES (2916111731090980863, 173813803857, "Gold", 999999999, 0, 0, 2, NULL, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
