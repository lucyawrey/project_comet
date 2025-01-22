-- Initial Migration for Creating Default Data
INSERT INTO world (id, name, logical_server) VALUES (545583615866591274, "Comet", "dev");
INSERT INTO world (id, name, logical_server) VALUES (545606222812829738, "Crystal", "dev");

INSERT INTO player (id, username, email, email_is_verified, role) VALUES (545602321573238826, "ProjectCometDev", "dev@lucyawrey.com", 1, 3);

INSERT INTO content_item (id, name, stack_size, type, icon_path, drop_model_path) VALUES (545607607855896618, "Gold", 999999999, 0, "/assets/icons/items/gold.png", "/assets/models/item_drops/gold.obj")
