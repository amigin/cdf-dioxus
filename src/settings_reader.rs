use serde::{Deserialize, Serialize};

use crate::grpc_client::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyNoSqlTcpReader")]
    pub my_no_sql_tcp_reader: String,

    #[serde(rename = "TraderCredsGrpcUrl")]
    pub trader_creds_grpc_url: String,

    #[serde(rename = "AccountsManagerGrpcUrl")]
    pub accounts_manager_grpc_url: String,

    #[serde(rename = "FavInstrumentsGrpcUrl")]
    pub fav_instruments_grpc_url: String,

    #[serde(rename = "KeyValueGrpcUrl")]
    pub key_value_grpc_url: String,

    #[serde(rename = "Brand")]
    pub brand: String,
}

impl SettingsReader {
    pub async fn get_brand(&self) -> String {
        let read_access = self.settings.read().await;
        return read_access.brand.clone();
    }
}

#[async_trait::async_trait]
impl my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_no_sql_tcp_reader.clone()
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TraderCredentialsGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.trader_creds_grpc_url.clone();
        }

        if name == AccountsManagerGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.accounts_manager_grpc_url.clone();
        }

        if name == FavoriteInstrumentsGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.fav_instruments_grpc_url.clone();
        }

        if name == KeyValueGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.key_value_grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
