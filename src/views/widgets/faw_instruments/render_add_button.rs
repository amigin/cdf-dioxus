use crate::{
    states::{AccountsState, GlobalState, MainFormState},
    types::InstrumentId,
    views::{icons::*, widgets::faw_instruments::select_instrument_panel},
};
use dioxus::prelude::*;

// save_instruments(cx, event_data.instruments, event_data.instrument_id);

pub fn render_add_button(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    if main_form_state.read().select_instrument_is_shown() {
        render! {
            table { style: "height: 40px;margin-left: 10px;",
                tr {
                    td {
                        div { style: "cursor: pointer; margin-left: 50px;", add_instrument_icon {} }
                        select_instrument_panel { on_click: move |(instrument, instruments)| { save_instruments(cx, instrument, instruments) } }
                    }
                }
            }
        }
    } else {
        render! {
            table { style: "height: 40px;margin-left: 10px;",
                tr {
                    td {
                        div {
                            style: "cursor: pointer;",
                            onclick: move |_| {
                                main_form_state.write().show_select_instrument();
                            },
                            add_instrument_icon {}
                        }
                    }
                }
            }
        }
    }
}

fn save_instruments(cx: &Scoped, instrument_id: InstrumentId, fav_instruments: Vec<InstrumentId>) {
    let trader_id = use_shared_state::<GlobalState>(cx)
        .unwrap()
        .read()
        .get_trader_id()
        .clone();

    let account_id = use_shared_state::<AccountsState>(cx)
        .unwrap()
        .read()
        .get_selected_account_id()
        .clone();

    cx.spawn(async move {
        tokio::spawn(async move {
            println!("Saving Fav instruments: {}", fav_instruments.len());
            crate::grpc_client::FavoriteInstrumentsGrpcClient::save_fav_instruments(
                trader_id.clone(),
                account_id.clone(),
                fav_instruments,
            )
            .await;

            println!("Saving selected instrument: {}", instrument_id);
            crate::grpc_client::KeyValueGrpcClient::save_selected_fav_instrument(
                trader_id,
                account_id,
                instrument_id,
            )
            .await;
        });
    });
}
