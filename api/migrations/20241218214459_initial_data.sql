-- Initial Migration for Creating Default Data
INSERT INTO world (id, name, logical_server) VALUES (545583615866591274, "Comet", "dev");
INSERT INTO world (id, name, logical_server) VALUES (545606222812829738, "Crystal", "dev");

INSERT INTO player (id, username, email, email_is_verified, role) VALUES (545602321573238826, "ProjectCometDev", "dev@lucyawrey.com", 1, 3);

INSERT INTO item (id, name, stack_size, is_unique, type, tradability, item_data, icon_path, drop_model_path) VALUES (545607607855896618, "Gold", 999999999, 0, 0, 2, NULL, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
