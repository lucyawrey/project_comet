tonic::include_proto!("game_data");

use crate::game_data_api::game_data_client::GameDataClient;
use tonic::transport::Channel;

pub async fn get_api_client() -> Result<GameDataClient<Channel>, tonic::transport::Error> {
    GameDataClient::connect("http://[::1]:50051").await
}
