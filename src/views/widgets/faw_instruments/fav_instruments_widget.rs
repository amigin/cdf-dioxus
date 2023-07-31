use dioxus::prelude::*;

use crate::{
    states::*,
    types::{AccountId, InstrumentId, TraderId},
    views::widgets::faw_instruments::render_instrument,
};

use super::render_add_button;

pub fn fav_instruments_widget(cx: Scope) -> Element {
    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let fav_instruments = fav_instruments_state.read();
    let selected = fav_instruments.get_selected();

    let instruments = use_shared_state::<InstrumentsState>(cx).unwrap().read();

    let mut fav_instruments: Vec<_> = fav_instruments
        .get_instruments()
        .iter()
        .map(|itm| Some(itm))
        .collect();

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();
    global_state.read();

    let accounts_state = use_shared_state::<AccountsState>(cx).unwrap();
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();
    //let account_id = accounts_state.read().get_selected_account_id();

    fav_instruments.push(None);

    let mut no = 0;

    render! {

        table { class: "tech-table",
            tr {

                fav_instruments.into_iter().map(|instrument_id| {
                    no+=1;


                    if let Some(instrument_id) = instrument_id{

                        let instrument = instruments.get(instrument_id);


                        let name = if let Some(instrument) = instrument {
                            instrument.name.clone()
                        } else {
                            instrument_id.to_string()
                        };



                        rsx!{
                            render_instrument{
                                id: instrument_id.clone(),
                                name: name,
                                selected: instrument_id.equals_to(selected),
                                no: no,
                                on_click: move |instr_id: InstrumentId| {
                                    fav_instruments_state.write().set_selected(instr_id.clone());
                                    main_form_state.write().hide_dialog();
                                    save_selected_fav_instrument(&cx,global_state.read().get_trader_id(), accounts_state.read().get_selected_account_id(), instr_id.clone());

                                },
                                on_remove: move |instr_id: InstrumentId| {
                                    let mut fav_instruments = fav_instruments_state.write();
                                    let new_list = fav_instruments.remove(instr_id.clone());
                                    save_instruments(&cx, global_state.read().get_trader_id(), accounts_state.read().get_selected_account_id(), new_list);
                                }
                            }
                        }
                    }else{
                        rsx!{
                            render_add_button{}
                         }
                    }


                })
            }
        }
    }
}

fn save_instruments(
    cx: &Scoped,
    trader_id: &TraderId,
    account_id: &AccountId,
    fav_instruments: Vec<InstrumentId>,
) {
    let trader_id = trader_id.clone();
    let account_id = account_id.clone();

    cx.spawn(async move {
        crate::grpc_client::FavoriteInstrumentsGrpcClient::save_fav_instruments(
            trader_id,
            account_id,
            fav_instruments,
        )
        .await;
    });
}

fn save_selected_fav_instrument(
    cx: &Scoped,
    trader_id: &TraderId,
    account_id: &AccountId,
    instrument_id: InstrumentId,
) {
    let trader_id = trader_id.clone();
    let account_id = account_id.clone();
    cx.spawn(async move {
        crate::grpc_client::KeyValueGrpcClient::save_selected_fav_instrument(
            trader_id,
            account_id,
            instrument_id,
        )
        .await;
    });
}
