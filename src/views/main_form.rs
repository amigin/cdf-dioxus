use crate::states::{AccountsState, GlobalState};

use super::*;
use dioxus::{
    html::{button, td},
    prelude::*,
};

pub fn main_form(cx: Scope) -> Element {
    let trader_id = {
        let global_state = use_shared_state::<GlobalState>(cx).unwrap();
        global_state.read().get_trader_id().clone()
    };

    let account_id = {
        let accounts_state = use_shared_state::<AccountsState>(cx).unwrap();
        accounts_state.read().get_selected_account_id().clone()
    };

    render! {
        div { id: "terminal-background",
            table { style: "width:100%; ",
                tr {
                    td { "LOGO" }
                    td { style: "width:100%" }
                    td {
                        div { style: "margin-top: 6px; margin-right: 6px;",
                            button { class: "btn btn-success btn-sm", "Deposit" }
                        }
                    }
                    td {
                        div { style: "margin-top: 6px; margin-right: 6px;",
                            button { class: "btn btn-outline-dark btn-sm", "Profile" }
                        }
                    }
                }
            }
            div { id: "terminal-pad",
                div { id: "fav-instruments", fav_instruments_panel { trader_id: trader_id, account_id: account_id } }

                div { id: "trading-panel",
                    table { class: "tech-table", style: "width:100%; height:100%;",
                        tr {
                            td { trading_panel {} }
                        }
                    }
                }
            }
        }
    }
}
