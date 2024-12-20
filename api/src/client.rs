use game_data::character_client::CharacterClient;
use game_data::create_character_request::{HomeWorld, Player};
use game_data::CreateCharacterRequest;
use std::env;

pub mod game_data {
    tonic::include_proto!("game_data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let character_name = args
        .get(1)
        .expect("Must provide a character name to create.");
    let home_default = "Comet".to_string();
    let player_default = "ProjectCometDev".to_string();
    let home_world = args.get(2).unwrap_or(&home_default);
    let player = args.get(3).unwrap_or(&player_default);
    let mut client = CharacterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreateCharacterRequest {
        name: character_name.into(),
        player: Some(Player::PlayerUsername(player.to_string())),
        home_world: Some(HomeWorld::HomeWorldName(home_world.to_string())),
    });

    let response = client.create(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
