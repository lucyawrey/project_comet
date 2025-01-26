tonic::include_proto!("game_data");

use crate::game_data_api::character_client::CharacterClient;
use tonic::transport::Channel;

pub async fn get_api_client() -> Result<CharacterClient<Channel>, tonic::transport::Error> {
    CharacterClient::connect("http://[::1]:50051").await
}
