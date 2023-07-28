use dioxus::prelude::UseSharedState;

use crate::{states::*, types::*};

pub async fn select_account(
    trader_id: &TraderId,
    account_id: &AccountId,
    fav_instruments_state: &UseSharedState<FavInstrumentsState>,
) {
    let fav_instruments = crate::grpc_client::FavoriteInstrumentsGrpcClient::get_fav_instruments(
        trader_id.clone(),
        account_id.clone(),
    )
    .await;

    let instrument_id = crate::grpc_client::KeyValueGrpcClient::get_fav_instrument(
        trader_id.clone(),
        account_id.clone(),
    )
    .await;

    fav_instruments_state
        .write()
        .set_fav_instruments(fav_instruments, instrument_id);
}
