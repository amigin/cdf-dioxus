use std::sync::Arc;

use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_nosql_contracts::*;
use tokio::sync::RwLock;

use crate::{grpc_client::*, settings_reader::SettingsReader};

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct MyNoSqlReaders {
    pub instruments: Arc<MyNoSqlDataReader<TradingInstrumentNoSqlEntity>>,
    pub instrument_avatars: Arc<MyNoSqlDataReader<InstrumentAvatarMyNoSqlEntity>>,
    pub defaults: Arc<MyNoSqlDataReader<DefaultsNoSqlEntity>>,
    pub bid_ask: Arc<MyNoSqlDataReader<BidAskSnapshotNoSqlEntity>>,
}

pub struct AppContextInner {
    pub trader_credentials_grpc_client: TraderCredentialsGrpcClient,
    pub accounts_manager_grpc_client: AccountsManagerGrpcClient,
    pub fav_instruments_grpc_client: FavoriteInstrumentsGrpcClient,
    pub key_value_grpc_client: KeyValueGrpcClient,
    pub settings_reader: Arc<SettingsReader>,

    pub my_no_sql_tcp_connection: MyNoSqlTcpConnection,
    pub readers: Arc<MyNoSqlReaders>,
}

impl AppContextInner {
    pub async fn get_brand(&self) -> String {
        self.settings_reader.get_brand().await
    }
}

impl AppContextInner {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        let my_no_sql_tcp_connection = MyNoSqlTcpConnection::new(APP_NAME, settings_reader.clone());

        let my_no_sql_readers = MyNoSqlReaders {
            instruments: my_no_sql_tcp_connection.get_reader().await,
            instrument_avatars: my_no_sql_tcp_connection.get_reader().await,
            defaults: my_no_sql_tcp_connection.get_reader().await,
            bid_ask: my_no_sql_tcp_connection.get_reader().await,
        };

        let result = Self {
            trader_credentials_grpc_client: TraderCredentialsGrpcClient::new(
                settings_reader.clone(),
            ),
            accounts_manager_grpc_client: AccountsManagerGrpcClient::new(settings_reader.clone()),
            fav_instruments_grpc_client: FavoriteInstrumentsGrpcClient::new(
                settings_reader.clone(),
            ),
            key_value_grpc_client: KeyValueGrpcClient::new(settings_reader.clone()),
            settings_reader,
            readers: Arc::new(my_no_sql_readers),
            my_no_sql_tcp_connection,
        };

        result
            .my_no_sql_tcp_connection
            .start(my_logger::LOGGER.clone())
            .await;

        result
    }
}

pub struct AppContext {
    pub inner: RwLock<Option<Arc<AppContextInner>>>,
    my_no_sql_readers: Option<MyNoSqlReaders>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
            my_no_sql_readers: None,
        }
    }

    pub async fn apply_settings(&self, settings_reader: Arc<SettingsReader>) {
        let inner_instance = AppContextInner::new(settings_reader).await;
        let mut inner = self.inner.write().await;
        *inner = Some(Arc::new(inner_instance));
    }

    pub async fn get(&self) -> Arc<AppContextInner> {
        let inner = self.inner.read().await;
        inner.as_ref().unwrap().clone()
    }

    pub async fn get_my_no_sql_readers(&self) -> Arc<MyNoSqlReaders> {
        let inner = self.inner.read().await;
        inner.as_ref().unwrap().readers.clone()
    }
}
