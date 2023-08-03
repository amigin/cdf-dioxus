use my_grpc_client_macros::generate_grpc_client;
use my_grpc_extensions::GrpcChannel;
use my_logger::LogEventCtx;
use my_telemetry::MyTelemetryContext;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::types::TraderId;

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
    pub async fn check_password(
        email: String,
        password: String,
    ) -> Result<TraderId, TraderCredentialsRequestFail> {
        let result = tokio::spawn(async move {
            let app = crate::APP_CTX.get().await;

            app.trader_credentials_grpc_client
                .verify_password(
                    VerifyTraderPasswordRequest {
                        email,
                        password,
                        brand: app.get_brand().await,
                    },
                    &MyTelemetryContext::new(),
                )
                .await

            //http::login(&username, &password).await
        })
        .await;

        if result.is_err() {
            my_logger::LOGGER.write_error(
                "Authentication",
                format!("Technical error on performing GRPC - check password"),
                LogEventCtx::new(),
            );
            return Err(TraderCredentialsRequestFail::TechnicalError);
        }

        let result = result.unwrap();

        if let Err(err) = result {
            my_logger::LOGGER.write_error(
                "Authentication",
                format!("Technical error on GRPC check password: {:?}", err),
                LogEventCtx::new(),
            );
            return Err(TraderCredentialsRequestFail::TechnicalError);
        }
        let result = result.unwrap();

        TraderCredentialsRequestFail::into_trader_id_response(result.status(), result.trader_id)
    }

    pub async fn register_new_client(
        email: String,
        password: String,
    ) -> Result<TraderId, TraderCredentialsRequestFail> {
        let result = tokio::spawn(async move {
            let app = crate::APP_CTX.get().await;

            let process_id = DateTimeAsMicroseconds::now().to_rfc3339();

            app.trader_credentials_grpc_client
                .register(
                    RegisterTraderRequest {
                        email: email,
                        password: password,
                        brand: app.get_brand().await,
                        process_id: process_id,
                    },
                    &MyTelemetryContext::new(),
                )
                .await
        })
        .await
        .unwrap();

        if let Err(err) = result {
            my_logger::LOGGER.write_error(
                "Registration",
                format!("Technical error on registration: {:?}", err),
                LogEventCtx::new(),
            );
            return Err(TraderCredentialsRequestFail::TechnicalError);
        }

        let result = result.unwrap();

        TraderCredentialsRequestFail::into_trader_id_response(result.status(), result.trader_id)
    }
}

pub enum TraderCredentialsRequestFail {
    TechnicalError,
    TraderExists,
    InvalidUsernameOrPassword,
    TraderNoFound,
    PasswordIsWrong,
}

impl TraderCredentialsRequestFail {
    pub fn into_trader_id_response(
        status: ResponseStatus,
        trader_id: Option<String>,
    ) -> Result<TraderId, Self> {
        match &status {
            ResponseStatus::Ok => Ok(trader_id.unwrap().into()),
            ResponseStatus::TraderExists => Err(Self::TraderExists),
            ResponseStatus::InvalidUsernameOrPassword => Err(Self::InvalidUsernameOrPassword),
            ResponseStatus::TraderNotFound => Err(Self::TraderNoFound),
            ResponseStatus::PasswordIsWrong => Err(Self::PasswordIsWrong),
        }
    }
}
