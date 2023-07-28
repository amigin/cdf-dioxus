use dioxus::prelude::*;

use crate::states::{AccountsState, FavInstrumentsState, GlobalState};
pub fn loading_form(cx: Scope) -> Element {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap().to_owned();

    let accounts_state = use_shared_state::<AccountsState>(cx).unwrap().to_owned();

    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx)
        .unwrap()
        .to_owned();

    let trader_id = global_state.read().get_trader_id().clone();
    cx.spawn(async move {
        let accounts =
            crate::grpc_client::AccountsManagerGrpcClient::get_list_of_accounts(trader_id.clone())
                .await;

        let selected_account = accounts[0].account_id.clone();

        accounts_state
            .write()
            .set_accounts(selected_account.clone(), accounts);

        let fav_instruments =
            crate::grpc_client::FavoriteInstrumentsGrpcClient::get_fav_instruments(
                trader_id.clone(),
                selected_account.clone(),
                crate::types::WebOrMobile::Web,
            )
            .await;

        let instrument_id = crate::grpc_client::KeyValueGrpcClient::get_fav_instrument(
            trader_id,
            selected_account.clone(),
        )
        .await;

        if let Some(instrument_id) = instrument_id {
            fav_instruments_state.write().selected = instrument_id;
        } else {
            if fav_instruments.len() > 0 {
                fav_instruments_state.write().selected = fav_instruments.get(0).unwrap().clone();
            }
        }

        fav_instruments_state.write().instruments = fav_instruments;

        global_state.write().set_authenticated();
    });

    //todo!("Implement working design")
    render! { h1 { "Loading..." } }
}
