use crate::{
    states::MainFormState,
    views::{icons::*, widgets::faw_instruments::select_instrument_widget},
};
use dioxus::prelude::*;

pub fn render_add_button(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    if main_form_state.read().select_instrument_is_shown() {
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
