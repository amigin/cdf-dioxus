use dioxus::prelude::*;

use crate::{
    states::FavInstrumentsState,
    types::{AccountId, InstrumentId, TraderId},
    views::icons::*,
};

#[derive(PartialEq, Props)]
pub struct FavInstrumentsProps {
    pub trader_id: TraderId,
    pub account_id: AccountId,
}

pub fn fav_instruments_panel<'s>(cx: Scope<'s, FavInstrumentsProps>) -> Element<'s> {
    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let fav_instruments = fav_instruments_state.read();
    let selected = &fav_instruments.get_selected();
    let mut fav_instruments: Vec<_> = fav_instruments
        .get_instruments()
        .iter()
        .map(|itm| Some(itm))
        .collect();

    fav_instruments.push(None);

    let mut no = 0;

    render! {

        table { class: "tech-table",
            tr {

                fav_instruments.into_iter().map(|item| {
                    no+=1;

                    if let Some(item) = item{

                        rsx!{
                            fav_instrument{
                                name: item.clone(),
                                selected: item.equals_to(selected),
                                no: no,
                                on_click: move |instr_id: InstrumentId| {
                                    save_fav_instrument(&cx, instr_id.clone());
                                    let mut fav_instruments = fav_instruments_state.write();
                                    fav_instruments.set_selected(instr_id.into());
                                },
                                on_remove: move |instr_id: InstrumentId| {
                                    let mut fav_instruments = fav_instruments_state.write();
                                    let new_list = fav_instruments.remove(instr_id.clone());
                                    save_instruments(cx, new_list);
                                }
                            }
                        }
                    }else{
                        rsx!{
                            table{ style:"height: 30px;margin-left: 10px;",
                                tr{
                                    td{
                                        div{ style:"cursor: pointer;",
                                            add_instrument_icon{}
                                        }

                                    }
                                }
                            }
                        }
                    }


                })
            }
        }
    }
}

#[derive(Props)]
pub struct FavInstrumentProps<'a> {
    pub name: InstrumentId,
    pub selected: bool,
    pub on_click: EventHandler<'a, InstrumentId>,
    pub on_remove: EventHandler<'a, InstrumentId>,
    pub no: usize,
}

fn fav_instrument<'s>(cx: Scope<'s, FavInstrumentProps<'s>>) -> Element<'s> {
    let first = if cx.props.no == 1 {
        "first-fav-instr"
    } else {
        ""
    };
    if cx.props.selected {
        return render! {
            td { div { class: "fav-instrument {first} selected", "{cx.props.name}" } }
            td { div { class: "fav-instr-separator" } }
        };
    }
    render! {
        td {
            table {
                tr {
                    td {
                        div {
                            class: "fav-instrument {first}",
                            onclick: move |_| {
                                cx.props.on_click.call(cx.props.name.clone());
                            },
                            "{cx.props.name.as_str()}"
                        }
                    }
                    td {
                        td {
                            div {
                                class: "hide_fav_instr",
                                onclick: move |_| {
                                    cx.props.on_remove.call(cx.props.name.clone());
                                },
                                close_icon {}
                            }
                        }
                    }
                }
            }
        }
        td { div { class: "fav-instr-separator" } }
    }
}

fn save_fav_instrument<'s>(cx: &'s Scoped<'s, FavInstrumentsProps>, instrument_id: InstrumentId) {
    let trader_id = cx.props.trader_id.clone();
    let account_id = cx.props.account_id.clone();
    cx.spawn(async move {
        crate::grpc_client::KeyValueGrpcClient::save_fav_instrument(
            trader_id,
            account_id,
            instrument_id,
        )
        .await;
    });
}

fn save_instruments<'s>(
    cx: &'s Scoped<'s, FavInstrumentsProps>,
    fav_instruments: Vec<InstrumentId>,
) {
    let trader_id = cx.props.trader_id.clone();
    let account_id = cx.props.account_id.clone();

    cx.spawn(async move {
        crate::grpc_client::FavoriteInstrumentsGrpcClient::save_fav_instruments(
            trader_id,
            account_id,
            fav_instruments,
        )
        .await;
    });
}
