use std::sync::{atomic::AtomicI32, Arc};

use tokio::sync::RwLock;

use crate::{grpc_client::*, settings_reader::SettingsReader};

pub struct AppContextInner {
    pub trader_credentials_grpc_client: TraderCredentialsGrpcClient,
    pub accounts_manager_grpc_client: AccountsManagerGrpcClient,
    pub fav_instruments_grpc_client: FavoriteInstrumentsGrpcClient,
    pub key_value_grpc_client: KeyValueGrpcClient,
    pub settings_reader: Arc<SettingsReader>,
}

impl AppContextInner {
    pub async fn get_brand(&self) -> String {
        self.settings_reader.get_brand().await
    }
}

impl AppContextInner {
    pub fn new(settings_reader: Arc<SettingsReader>) -> Self {
        Self {
            trader_credentials_grpc_client: TraderCredentialsGrpcClient::new(
                settings_reader.clone(),
            ),
            accounts_manager_grpc_client: AccountsManagerGrpcClient::new(settings_reader.clone()),
            fav_instruments_grpc_client: FavoriteInstrumentsGrpcClient::new(
                settings_reader.clone(),
            ),
            key_value_grpc_client: KeyValueGrpcClient::new(settings_reader.clone()),
            settings_reader,
        }
    }
}

pub struct AppContext {
    pub inner: RwLock<Option<Arc<AppContextInner>>>,
    pub test_data: AtomicI32,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
            test_data: AtomicI32::new(0),
        }
    }

    pub async fn apply_settings(&self, settings_reader: Arc<SettingsReader>) {
        let mut inner = self.inner.write().await;
        *inner = Some(Arc::new(AppContextInner::new(settings_reader)));
    }

    pub async fn get(&self) -> Arc<AppContextInner> {
        let inner = self.inner.read().await;
        inner.as_ref().unwrap().clone()
    }

    pub fn get_test_value(&self) -> i32 {
        self.test_data.load(std::sync::atomic::Ordering::Relaxed)
    }
}
