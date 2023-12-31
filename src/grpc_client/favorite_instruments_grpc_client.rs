use my_grpc_client_macros::generate_grpc_client;
use my_grpc_extensions::GrpcChannel;
use my_telemetry::MyTelemetryContext;

use crate::{
    types::{AccountId, InstrumentId, TraderId},
    APP_CTX,
};

#[generate_grpc_client(
    proto_file = "./proto/FavoriteInstrumentsFlows.proto",
    crate_ns: "crate::favorite_instruments_flows_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct FavoriteInstrumentsGrpcClient {
    channel: GrpcChannel<TGrpcService>,
}

impl FavoriteInstrumentsGrpcClient {
    pub async fn get_fav_instruments(
        trader_id: TraderId,
        account_id: AccountId,
    ) -> Vec<InstrumentId> {
        let result = tokio::spawn(async move {
            let app_ctx: std::sync::Arc<crate::app::AppContextInner> = APP_CTX.get().await;
            let result = app_ctx
                .fav_instruments_grpc_client
                .get(
                    GetFavoriteInstrumentsRequestModel {
                        trader_id: trader_id.into(),
                        account_id: account_id.into(),
                        web_or_mobile: 0,
                    },
                    &MyTelemetryContext::new(),
                )
                .await
                .unwrap();

            result
        })
        .await
        .unwrap();

        result
            .instruments
            .into_iter()
            .map(|itm| itm.into())
            .collect()
    }

    pub async fn save_fav_instruments(
        trader_id: TraderId,
        account_id: AccountId,
        instruments: Vec<InstrumentId>,
    ) {
        tokio::spawn(async move {
            let app_ctx: std::sync::Arc<crate::app::AppContextInner> = APP_CTX.get().await;

            let mut result = Vec::with_capacity(instruments.len());

            for itm in instruments {
                result.push(itm.into_string())
            }

            let req = SetFavoriteInstrumentsRequestModel {
                trader_id: trader_id.into(),
                account_id: account_id.into(),
                web_or_mobile: 0,
                instruments: result,
            };

            let _ = app_ctx
                .fav_instruments_grpc_client
                .set(req, &MyTelemetryContext::new())
                .await;
        })
        .await
        .unwrap();
    }
}
