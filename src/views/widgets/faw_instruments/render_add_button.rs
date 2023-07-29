use crate::{
    view_models::*,
    views::{icons::*, widgets::faw_instruments::select_instrument_widget},
};
use dioxus::prelude::*;

pub fn render_add_button(cx: Scope) -> Element {
    let select_instrument_view_model = use_shared_state::<SelectInstrumentViewModel>(cx).unwrap();

    if select_instrument_view_model.read().show {
        render! {
            table { style: "height: 40px;margin-left: 10px;",
                tr {
                    td {
                        div { style: "cursor: pointer; margin-left: 50px;", add_instrument_icon {} }

                        select_instrument_widget {}
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
                                select_instrument_view_model.write().show = true;
                            },
                            add_instrument_icon {}
                        }
                    }
                }
            }
        }
    }
}
