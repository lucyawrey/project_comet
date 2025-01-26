mod game_data_api;
use game_data_api::create_character_request::{HomeWorld, Player};
use game_data_api::get_api_client;
use game_data_api::CreateCharacterRequest;
use std::env;

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
    let mut client = get_api_client().await?;

    let request = tonic::Request::new(CreateCharacterRequest {
        name: character_name.into(),
        player: Some(Player::PlayerUsername(player.to_string())),
        home_world: Some(HomeWorld::HomeWorldName(home_world.to_string())),
    });

    let response = client.create(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
