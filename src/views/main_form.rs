use crate::states::*;
use crate::types::*;

use super::widgets::*;
use super::*;
use crate::views::icons::*;
use dioxus::prelude::*;

pub fn main_form(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainFormState::new());

    let trader_id = {
        let global_state = use_shared_state::<GlobalState>(cx).unwrap();
        global_state.read().get_trader_id().clone()
    };

    let account_id = {
        let accounts_state = use_shared_state::<AccountsState>(cx).unwrap();
        accounts_state.read().get_selected_account_id().clone()
    };

    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let trader_id_on_click = trader_id.clone();
    render! {
        div { id: "terminal-background",
            table { style: "width:100%; ",
                tr {
                    td { "LOGO" }
                    td { style: "width:100%" }
                    td { account_balance_widget {} }
                    td {
                        div { style: "margin-top: 6px; margin-right: 6px;",
                            button { class: "btn btn-success btn-sm", "Deposit" }
                        }
                    }
                    td {
                        div { style: "margin-top: 6px; margin-right: 6px;",
                            button { class: "btn btn-outline-dark btn-sm dropdown-toggle",

                                span {
                                    person_icon {}
                                    "Profile"
                                }
                            }
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
        select_account_widget {
            on_account_selected: move |account_id: AccountId| {
                let accounts_state = use_shared_state::<AccountsState>(cx).unwrap();
                let account_id = account_id.clone();
                select_account(
                    &cx,
                    trader_id_on_click.clone(),
                    account_id,
                    accounts_state,
                    fav_instruments_state,
                );
            }
        }
    }
}

fn select_account(
    cx: &Scoped,
    trader_id: TraderId,
    account_id: AccountId,
    accounts_state: &UseSharedState<AccountsState>,
    fav_instruments_state: &UseSharedState<FavInstrumentsState>,
) {
    let accounts_state = accounts_state.to_owned();

    let fav_instruments_state = fav_instruments_state.to_owned();

    cx.spawn(async move {
        crate::actions::select_account(&trader_id, &account_id, &fav_instruments_state).await;

        crate::grpc_client::KeyValueGrpcClient::save_selected_account_id(
            trader_id.clone(),
            account_id.clone(),
        )
        .await;

        accounts_state
            .write()
            .set_selected_account(account_id.clone());
    });
}
