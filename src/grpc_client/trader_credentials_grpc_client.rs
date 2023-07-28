use my_grpc_client_macros::generate_grpc_client;
use my_grpc_extensions::GrpcChannel;
use my_telemetry::MyTelemetryContext;

use crate::types::{RequestFail, TraderId};

#[generate_grpc_client(
    proto_file = "./proto/TraderCredentialsGrpcService.proto",
    crate_ns: "crate::trader_credentials_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct TraderCredentialsGrpcClient {
    channel: GrpcChannel<TGrpcService>,
}

impl TraderCredentialsGrpcClient {
    pub async fn check_password(email: String, password: String) -> Result<TraderId, RequestFail> {
        let result = tokio::spawn(async move {
            let app = crate::APP_CTX.get().await;

            let result = app
                .trader_credentials_grpc_client
                .verify_password(
                    VerifyTraderPasswordRequest {
                        email,
                        password,
                        brand: app.get_brand().await,
                    },
                    &MyTelemetryContext::new(),
                )
                .await
                .unwrap();

            result.trader_id

            //http::login(&username, &password).await
        })
        .await
        .unwrap();

        match result {
            Some(value) => Ok(value.into()),
            None => Err(RequestFail::InvalidUserNameOrPassword),
        }
    }
}
