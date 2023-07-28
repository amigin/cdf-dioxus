use dioxus::prelude::*;

use crate::{
    states::FavInstrumentsState,
    types::{AccountId, InstrumentId, TraderId},
};

#[derive(PartialEq, Props)]
pub struct FavInstrumentsProps {
    pub trader_id: TraderId,
    pub account_id: AccountId,
}

pub fn fav_instruments_panel<'s>(cx: Scope<'s, FavInstrumentsProps>) -> Element<'s> {
    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let fav_instruments = fav_instruments_state.read();
    let selected = &fav_instruments.selected;
    let fav_instruments = fav_instruments.instruments.as_slice();

    let mut no = 0;

    render! {

        table { class: "tech-table",
            tr {

                fav_instruments.iter().map(|item| {
                    no+=1;

                    rsx!{
                        fav_instrument{
                            name: item.clone(),
                            selected: item.equals_to(selected),
                            no: no,
                            on_click: move |name: InstrumentId| {
                                save_fav_instrument(&cx, cx.props.trader_id.clone(), cx.props.account_id.clone(), name.clone());
                                let mut fav_instruments = fav_instruments_state.write();
                                fav_instruments.selected = name.into();
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
            div {
                class: "fav-instrument {first}",
                onclick: move |_| {
                    cx.props.on_click.call(cx.props.name.clone());
                },
                "{cx.props.name.as_str()}"
            }
        }
        td { div { class: "fav-instr-separator" } }
    }
}

fn save_fav_instrument<'s>(
    cx: &'s Scoped<'s, FavInstrumentsProps>,
    trader_id: TraderId,
    account_id: AccountId,
    instrument_id: InstrumentId,
) {
    cx.spawn(async move {
        crate::grpc_client::KeyValueGrpcClient::save_fav_instrument(
            trader_id,
            account_id,
            instrument_id,
        )
        .await;
    });
}
