use dioxus::prelude::*;

use crate::{
    types::*,
    views::{
        icons::*,
        widgets::faw_instruments::{render_avatar, render_rate},
    },
};

#[derive(Props)]
pub struct FavInstrumentProps<'a> {
    pub id: InstrumentId,
    pub name: String,
    pub selected: bool,
    pub on_click: EventHandler<'a, InstrumentId>,
    pub on_remove: EventHandler<'a, InstrumentId>,
    pub no: usize,
}

pub fn render_instrument<'s>(cx: Scope<'s, FavInstrumentProps<'s>>) -> Element<'s> {
    let first = if cx.props.no == 1 {
        "first-fav-instr"
    } else {
        ""
    };
    if cx.props.selected {
        return render! {
            td {
                table { class: "fav-instrument {first} selected",
                    tr {
                        td { rowspan: 2, render_avatar { id: cx.props.id.as_str().into() } }
                        td { style: "width: 100%;", "{cx.props.name}" }
                        td { rowspan: 2,
                            div { style: "opacity:0", class: "hide_fav_instr", close_icon {} }
                        }
                    }
                    tr { class: "fav-instr-rate", render_rate { instrument_id: cx.props.id.clone() } }
                }
            }
            td { div { class: "fav-instr-separator" } }
        };
    }
    render! {
        td {
            table {
                class: "fav-instrument {first}",
                onclick: move |_| {
                    cx.props.on_click.call(cx.props.id.clone());
                },
                tr {
                    td {
                        table {
                            tr {
                                td { rowspan: 2, render_avatar { id: cx.props.id.as_str().into() } }
                                td { style: "width: 100%;", "{cx.props.name.as_str()}" }
                                td { rowspan: 2,
                                    div {
                                        class: "hide_fav_instr",
                                        onclick: move |_| {
                                            cx.props.on_remove.call(cx.props.id.clone());
                                        },
                                        close_icon {}
                                    }
                                }
                            }
                            tr {
                                tr { class: "fav-instr-rate", render_rate { instrument_id: cx.props.id.clone() } }
                            }
                        }
                    }
                }
            }
        }
        td { div { class: "fav-instr-separator" } }
    }
}
