use dioxus::prelude::*;

use crate::{
    states::*,
    types::{AccountId, InstrumentId, TraderId},
    views::widgets::faw_instruments::render_instrument,
};

use super::render_add_button;

#[derive(PartialEq, Props)]
pub struct FavInstrumentsProps {
    pub trader_id: TraderId,
    pub account_id: AccountId,
}

pub fn fav_instruments_widget<'s>(cx: Scope<'s, FavInstrumentsProps>) -> Element<'s> {
    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let fav_instruments = fav_instruments_state.read();
    let selected = &fav_instruments.get_selected();

    let instruments = use_shared_state::<InstrumentsState>(cx).unwrap().read();

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
                                    save_selected_fav_instrument(&cx, instr_id.clone());
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
                            render_add_button{}
                         }
                    }


                })
            }
        }
    }
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

fn save_selected_fav_instrument<'s>(
    cx: &'s Scoped<'s, FavInstrumentsProps>,
    instrument_id: InstrumentId,
) {
    let trader_id = cx.props.trader_id.clone();
    let account_id = cx.props.account_id.clone();
    cx.spawn(async move {
        crate::grpc_client::KeyValueGrpcClient::save_selected_fav_instrument(
            trader_id,
            account_id,
            instrument_id,
        )
        .await;
    });
}
