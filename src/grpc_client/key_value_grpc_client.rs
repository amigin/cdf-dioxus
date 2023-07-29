use std::collections::HashMap;

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
    async fn get_key(client_id: String, key: String) -> Option<String> {
        let result = tokio::spawn(async move {
            let app_ctx = APP_CTX.get().await;

            let req = GetKeyValueGrpcRequestModel {
                client_id: client_id,
                key: key,
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
                        return result.value;
                    }
                }
                None => {
                    return None;
                }
            }

            None
        })
        .await;

        match result {
            Ok(result) => result,
            Err(_) => None,
        }
    }

    /*
       async fn get_keys(client_id: String, keys: Vec<String>) -> Option<HashMap<String, String>> {
           let result = tokio::spawn(async move {
               let app_ctx = APP_CTX.get().await;

               let mut req = Vec::with_capacity(keys.len());

               for key in &keys {
                   let model = GetKeyValueGrpcRequestModel {
                       client_id: client_id.to_string(),
                       key: key.clone(),
                   };
                   req.push(model);
               }

               let response = app_ctx
                   .key_value_grpc_client
                   .get(req, &my_telemetry::MyTelemetryContext::new())
                   .await
                   .unwrap();

               if response.is_none() {
                   return None;
               }

               let response = response.unwrap();

               let mut result = HashMap::new();

               for itm in response {
                   if let Some(value) = itm.value {
                       result.insert(itm.key, value);
                   }
               }

               Some(result)
           })
           .await;

           match result {
               Ok(result) => result,
               Err(_) => None,
           }
       }
    */
    async fn save_key(client_id: String, key: String, value: String) {
        let _ = tokio::spawn(async move {
            let app_ctx = APP_CTX.get().await;

            let req = SetKeyValueGrpcRequestModel {
                client_id,
                key,
                value,
            };

            app_ctx
                .key_value_grpc_client
                .set(vec![req], &my_telemetry::MyTelemetryContext::new())
                .await
                .unwrap()
        })
        .await;
    }

    pub async fn get_fav_instrument(
        trader_id: TraderId,
        account_id: AccountId,
    ) -> Option<InstrumentId> {
        let result = Self::get_key(trader_id.into(), compile_fav_instrument_key(&account_id)).await;
        match result {
            Some(itm) => {
                return Some(itm.into());
            }
            None => None,
        }
    }

    pub async fn save_selected_fav_instrument(
        trader_id: TraderId,
        account_id: AccountId,
        instrument_id: InstrumentId,
    ) {
        Self::save_key(
            trader_id.into(),
            compile_fav_instrument_key(&account_id),
            instrument_id.into(),
        )
        .await;
    }

    pub async fn get_selected_instrument(
        trader_id: TraderId,
        account_id: AccountId,
    ) -> Option<InstrumentId> {
        let result = Self::get_key(trader_id.into(), compile_fav_instrument_key(&account_id)).await;
        match result {
            Some(itm) => {
                return Some(itm.into());
            }
            None => None,
        }
    }

    pub async fn get_selected_account_id(trader_id: TraderId) -> Option<AccountId> {
        let result = Self::get_key(trader_id.into(), KEY_SELECTED_ACCOUNT.to_string()).await;
        match result {
            Some(itm) => {
                return Some(itm.into());
            }
            None => None,
        }
    }

    pub async fn save_selected_account_id(trader_id: TraderId, account_id: AccountId) {
        Self::save_key(
            trader_id.into(),
            KEY_SELECTED_ACCOUNT.to_string(),
            account_id.into(),
        )
        .await;
    }
}

fn compile_fav_instrument_key(account_id: &AccountId) -> String {
    format!("fav-instr-{}", account_id)
}

const KEY_SELECTED_ACCOUNT: &str = "selected-account";
