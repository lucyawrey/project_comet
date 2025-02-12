-- Initial Migration for Creating Default Data
INSERT INTO game_server (id, created_at, updated_at, region_code, display_name) VALUES ("development", 173842930888, 173842930888, "US/Eastern", "Development");

INSERT INTO world (id, created_at, updated_at, game_server_id, display_name) VALUES ("clockwork", 173842930888, 173842930888, "development", "Clockwork");

INSERT INTO user (id, updated_at, username, role) VALUES (2916600401581113343, 173842930888, "CometAdmin", 4);

INSERT INTO content (id, updated_at, name, content_type, content_subtype, data) VALUES (2916600401581178879, 173842930888, "Gold", 0, 0, '{"stack_size":"999999999","tradability":1,"is_unique":false,"is_soulbound":false}')
