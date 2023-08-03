use crate::{states::*, APP_CTX};
use dioxus::prelude::*;
use my_nosql_contracts::TradingInstrumentNoSqlEntity;
pub fn loading_form(cx: Scope) -> Element {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let global_state_owned = global_state.to_owned();

    let accounts_state = use_shared_state::<AccountsState>(cx).unwrap().to_owned();

    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx)
        .unwrap()
        .to_owned();

    let instruments_state = use_shared_state::<InstrumentsState>(cx).unwrap().to_owned();

    let (trader_id, email) = {
        let read = global_state.read();
        (read.get_trader_id().clone(), read.get_email())
    };

    cx.spawn(async move {
        let accounts =
            crate::grpc_client::AccountsManagerGrpcClient::get_list_of_accounts(trader_id.clone())
                .await;

        let selected_account_id_from_key_value =
            crate::grpc_client::KeyValueGrpcClient::get_selected_account_id(trader_id.clone())
                .await;

        let mut selected_account_id = accounts[0].account_id.clone();

        if let Some(selected_account_id_from_key_value) = selected_account_id_from_key_value {
            for itm in &accounts {
                if itm.account_id == selected_account_id {
                    selected_account_id = selected_account_id_from_key_value;
                    break;
                }
            }
        }

        accounts_state
            .write()
            .set_accounts(selected_account_id.clone(), accounts);

        crate::actions::select_account(&trader_id, &selected_account_id, &fav_instruments_state)
            .await;
        let my_no_sql_readers = APP_CTX.get_my_no_sql_readers().await;

        let instruments = my_no_sql_readers
            .instruments
            .get_by_partition_key_as_vec(TradingInstrumentNoSqlEntity::generate_partition_key())
            .await;

        if let Some(instruments) = instruments {
            instruments_state.write().set_instruments(instruments);
        }

        global_state_owned.write().set_authenticated();
    });

    //todo!("Implement working design")
    render! {
        table { style: "width:100%; height:100%;",
            tr {
                td {
                    div { class: "spinner-border", role: "status", span { class: "visually-hidden", "Loading..." } }
                }
            }
        }

        script { "localStorage.setItem('email', '{email}')" }
    }
}
