use my_grpc_client_macros::generate_grpc_client;
use my_grpc_extensions::GrpcChannel;
use my_telemetry::MyTelemetryContext;

use crate::{
    types::{TraderAccount, TraderId},
    APP_CTX,
};

#[generate_grpc_client(
    proto_file = "./proto/AccountsManagerGrpcService.proto",
    crate_ns: "crate::accounts_manager_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct AccountsManagerGrpcClient {
    channel: GrpcChannel<TGrpcService>,
}

impl AccountsManagerGrpcClient {
    pub async fn get_list_of_accounts(trader_id: TraderId) -> Vec<TraderAccount> {
        let result = tokio::spawn(async move {
            let app_ctx: std::sync::Arc<crate::app::AppContextInner> = APP_CTX.get().await;
            let result = app_ctx
                .accounts_manager_grpc_client
                .get_client_accounts(
                    AccountManagerGetClientAccountsGrpcRequest {
                        trader_id: trader_id.into(),
                    },
                    &MyTelemetryContext::new(),
                )
                .await
                .unwrap();

            result
        });

        match result.await.unwrap() {
            Some(response) => response
                .into_iter()
                .map(|account| TraderAccount {
                    account_id: account.id.into(),
                    currency: account.currency.into(),
                })
                .collect(),
            None => {
                vec![]
            }
        }
    }
}
