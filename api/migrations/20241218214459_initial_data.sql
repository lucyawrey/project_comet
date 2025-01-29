-- Initial Migration for Creating Default Data
INSERT INTO logical_server (id, created_at, updated_at, name) VALUES ("dev", 173817452843, 173817452843, "Development Server");
INSERT INTO world (id, updated_at, name, logical_server_id) VALUES (2916111731091177471, 173813803857, "Comet", "dev");
INSERT INTO world (id, updated_at, name, logical_server_id) VALUES (2916111731091111935, 173813803857, "Crystal", "dev");

INSERT INTO player (id, updated_at, username, email, email_is_verified, role) VALUES (2916111731091046399, 173813803857, "ProjectCometDev", "dev@lucyawrey.com", 1, 3);

INSERT INTO item (id, updated_at, name, stack_size, is_unique, type, tradability, data, icon_path, drop_model_path) VALUES (2916111731090980863, 173813803857, "Gold", 999999999, 0, 0, 2, NULL, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
