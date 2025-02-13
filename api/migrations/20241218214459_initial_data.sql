-- Initial Migration for Creating Default Data
INSERT INTO game_server (id, region_code, display_name) VALUES ("development", "US/Eastern", "Development");

INSERT INTO world (id, game_server_id, display_name) VALUES ("clockwork", "development", "Clockwork");

INSERT INTO user (id, username, role) VALUES (2916600401581113343, "CometAdmin", 4);

INSERT INTO content (id, name, content_type, content_subtype, data) VALUES (2916600401581178879, "Gold", 0, 0, '{"stack_size":"999999999","tradability":1,"is_unique":false,"is_soulbound":false}')
