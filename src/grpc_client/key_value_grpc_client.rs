use crate::{
    types::{AccountId, InstrumentId, TraderId},
    APP_CTX,
};

#[my_grpc_client_macros::generate_grpc_client(
    proto_file: "./proto/KeyValueFlows.proto",
    crate_ns: "crate::keyvalue_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct KeyValueGrpcClient {
    channel: my_grpc_extensions::GrpcChannel<TGrpcService>,
}

impl KeyValueGrpcClient {
    pub async fn get_fav_instrument(
        trader_id: TraderId,
        account_id: AccountId,
    ) -> Option<InstrumentId> {
        let result = tokio::spawn(async move {
            let app_ctx = APP_CTX.get().await;

            let req = GetKeyValueGrpcRequestModel {
                client_id: trader_id.into(),
                key: compile_key(&account_id),
            };

            let result = app_ctx
                .key_value_grpc_client
                .get(vec![req], &my_telemetry::MyTelemetryContext::new())
                .await
                .unwrap();

            match result {
                Some(mut result) => {
                    if result.len() > 0 {
                        let result = result.remove(0);
                        if let Some(value) = result.value {
                            return Some(InstrumentId::new(value));
                        }
                    }
                }
                None => {
                    return None;
                }
            }

            None

            /*
            let key = compile_key(&account_id);


            let result = app_ctx
                .key_value_grpc_client
                .get_all_by_user(
                    GetAllByUserGrpcRequestModel {
                        client_id: trader_id.into(),
                    },
                    &my_telemetry::MyTelemetryContext::new(),
                )
                .await
                .unwrap();

            if let Some(result) = result {
                for itm in result {
                    if itm.key == key {
                        return itm.value;
                    }
                }
                None
            } else {
                None
            } */
        })
        .await
        .unwrap();

        match result {
            Some(itm) => {
                return Some(itm.into());
            }
            None => None,
        }
    }

    pub async fn save_fav_instrument(
        trader_id: TraderId,
        account_id: AccountId,
        instrument_id: InstrumentId,
    ) {
        tokio::spawn(async move {
            let app_ctx = APP_CTX.get().await;

            let req = SetKeyValueGrpcRequestModel {
                client_id: trader_id.into(),
                key: compile_key(&account_id),
                value: instrument_id.into(),
            };

            app_ctx
                .key_value_grpc_client
                .set(vec![req], &my_telemetry::MyTelemetryContext::new())
                .await
                .unwrap()
        })
        .await
        .unwrap();
    }
}

fn compile_key(account_id: &AccountId) -> String {
    format!("fav-instr-{}", account_id)
}
