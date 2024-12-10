use game_data::character_client::CharacterClient;
use game_data::CreateCharacterRequest;

pub mod game_data {
    tonic::include_proto!("game_data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CharacterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreateCharacterRequest {
        name: "Luci Relanah".into(),
        player_id: 1,
    });

    let response = client.create(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
