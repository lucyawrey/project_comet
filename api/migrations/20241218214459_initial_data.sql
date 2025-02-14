-- Initial Migration for Creating Default Data
INSERT INTO game_server (id, region_code, display_name) VALUES ("development", "US/Eastern", "Development");
INSERT INTO world (id, game_server_id, display_name) VALUES ("clockwork", "development", "Clockwork");
