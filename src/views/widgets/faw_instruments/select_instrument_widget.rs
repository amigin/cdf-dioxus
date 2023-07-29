use crate::{
    states::*,
    types::*,
    view_models::*,
    views::{
        icons::*,
        widgets::faw_instruments::{render_avatar, render_rate},
    },
};
use dioxus::prelude::*;

pub fn select_instrument_widget(cx: Scope) -> Element {
    let select_instrument_view_model = use_shared_state::<SelectInstrumentViewModel>(cx).unwrap();

    let filter = use_state(cx, || "".to_string());

    let mut instruments = use_shared_state::<InstrumentsState>(cx)
        .unwrap()
        .read()
        .instruments
        .clone();

    let fav_instruments = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    for fav_instr in fav_instruments.read().get_instruments() {
        instruments.remove(fav_instr.as_str());
    }

    let filter_value = filter.get().trim().to_lowercase();

    if filter_value.len() > 0 {
        instruments.retain(|_, instrument| {
            instrument.name.to_lowercase().contains(&filter_value)
                || instrument
                    .instrument_id
                    .as_str()
                    .to_lowercase()
                    .contains(&filter_value)
        });
    }

    render! {
        div { id: "selectInstrumentModal",
            div { id: "selectInstrumentSearchPanel",
                div { class: "search-icon", instrument_search_icon {} }
                div { class: "input-group input-group-sm",
                    input {
                        id: "selectInstrument",
                        class: "form-control, edit-underline",
                        oninput: |e| {
                            filter.set(e.value.clone());
                        }
                    }
                }
                div {
                    class: "close-icon",
                    onclick: move |_| {
                        select_instrument_view_model.write().show = false;
                    },
                    close_icon {}
                }
            }
            div { id: "selectInstrumentList",

                instruments.into_iter().map(|(_, instrument)| {
                    let name = instrument.name.clone();
                    let id = instrument.instrument_id.as_str().to_string();

                    let id_on_click = id.clone();


                    rsx!{
                        div {
                            class: "select-instrument",
                            onclick: move |_| {
                                let instr = fav_instruments.write().add(id_on_click.clone().into());
                                select_instrument_view_model.write().show = false;
                                save_instruments(cx, instr);
                            },
                            div{class: "instrument-item instrument-avatar",
                                render_avatar{id: id.clone()}
                            }
                            div{class: "instrument-item instrument-name",
                               div{
                                  div{name}
                                  div{ class:"instrument-id",  id}
                                }
                            }
                            div{class: "instrument-item instrument-rate fav-instr-rate",
                                render_rate{instrument_id: instrument.instrument_id.clone()},
                            }

                        }
                    }
                })
            }
        }
        script { "set_focus('selectInstrument')" }
    }
}

fn save_instruments(cx: &Scoped, fav_instruments: Vec<InstrumentId>) {
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
        crate::grpc_client::FavoriteInstrumentsGrpcClient::save_fav_instruments(
            trader_id,
            account_id,
            fav_instruments,
        )
        .await;
    });
}
