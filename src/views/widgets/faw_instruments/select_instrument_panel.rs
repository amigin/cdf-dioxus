use crate::{
    states::*,
    types::*,
    views::{
        icons::*,
        widgets::faw_instruments::{render_avatar, render_rate},
    },
};
use dioxus::{html::input_data::keyboard_types::Code, prelude::*};
#[derive(Props)]
pub struct SelectInstrumentProps<'s> {
    pub on_click: EventHandler<'s, (InstrumentId, Vec<InstrumentId>)>,
}

pub fn select_instrument_panel<'s>(cx: Scope<'s, SelectInstrumentProps>) -> Element<'s> {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

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
        div { id: "selectInstrumentModal", class: "floating-menu",
            div { id: "selectInstrumentSearchPanel",
                div { class: "search-icon", instrument_search_icon {} }
                div { class: "input-group input-group-sm",
                    input {
                        id: "selectInstrument",
                        class: "form-control, edit-underline",
                        onkeyup: |e| {
                            if let Code::Escape = e.data.code() {
                                main_form_state.write().hide_dialog();
                            }
                        },
                        oninput: |e| {
                            filter.set(e.value.clone());
                        }
                    }
                }
                div {
                    class: "close-icon",
                    onclick: move |_| {
                        main_form_state.write().hide_dialog();
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
                                main_form_state.write().hide_dialog();
                                cx.props.on_click.call((
                                    id_on_click.clone().into(),
                                    instr,
                                ));
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
