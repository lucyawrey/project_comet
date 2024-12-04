# EXTREMELY EARLY BEVY MMO TEST

## Projects
- [ ] Client - Bevy game
- [ ] Server - [bevy_replicon](https://github.com/projectharmonia/bevy_replicon) over bevy_renet2 for WebTransport (in browser)
- [ ] API - Web server for handling data using gRPC (Tonic?), SQLX and SQLite. Should support huge amounts of player data. Examples: essentially infinite 'item box' capable of holding at least one of every equipment, collectible and item stack in the game (each update will add more); player progress on every quest in the game; player progress on classes and professions including skill trees and cross class skills; character customization data including base character creation data as well as equipped gear and even custom textures (likely small pixel art textures saved to the DB as a blobs)