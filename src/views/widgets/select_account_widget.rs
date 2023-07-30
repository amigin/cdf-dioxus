use dioxus::prelude::*;

use crate::{
    states::{AccountsState, MainFormState},
    types::AccountId,
    views::{icons::*, widgets::real_demo_badge},
};

#[derive(Props)]
pub struct SelectAccountsProps<'a> {
    pub on_account_selected: EventHandler<'a, AccountId>,
}

pub fn select_account_widget<'s>(cx: Scope<'s, SelectAccountsProps<'s>>) -> Element<'s> {
    let accounts = use_shared_state::<AccountsState>(cx).unwrap();

    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    let select_account_is_shown = main_form_state.read().select_account_is_shown();

    let accounts = accounts.read();
    let selected_account = accounts.get_selected_account();
    let accounts = accounts.get_accounts();

    let mut result = Vec::new();

    if select_account_is_shown {
        let content = rsx! {
            div {
                id: "dialogBackground",
                onclick: move |_| {
                    main_form_state.write().hide_dialog();
                },
                table { style: "width:100%; height:100%;text-align: left;",
                    tr {
                        td { style: "vertical-align:top;",
                            div { id: "selectAccountPanel",
                                table { id: "selectAccountContent",
                                    accounts.iter().map(|itm|{
                                        let account_id = itm.account_id.as_str().to_string();
                                        let account_id2 = itm.account_id.as_str().to_string();
                                        let currency = itm.currency.as_str();
                                        let balance = itm.balance.to_string();
                                        let is_live = itm.is_live;

                                        let is_selected = itm.is_same_account(selected_account);
                                        let style = if is_selected {
                                            "background: #f0ffe9;"
                                        }else{
                                            "cursor: pointer;"
                                        };


                                        render!{
                                            tr{ style:"{style}",
                                            onclick: move |_| {
                                                cx.props.on_account_selected.call(account_id.clone().into());
                                                main_form_state.write().hide_dialog();
                                             },
                                                td{rowspan:"2", style:"text-align: center;",
                                                markets_icon{}

                                                }
                                                td{
                                                    "{currency}{balance}", real_demo_badge { is_live: is_live }
                                                }td{
                                                    render_invested{selected: is_selected, currency:currency.to_string()}
                                                }td{
                                                    render_profit{selected: is_selected, currency:currency.to_string()}
                                                }
                                                td{
                                                    render_available{selected: is_selected, currency:currency.to_string()}
                                                }
                                                td{rowspan:"2",
                                                    button{
                                                        class:"btn btn-success",
                                                        "Deposit"
                                                    }
                                                }
                                                td{rowspan:"2",
                                                    button{
                                                        class:"btn btn-light",
                                                        "Withdraw"
                                                    }
                                                }

                                            }
                                            tr{ style:"{style}",
                                            onclick: move |_| {
                                                cx.props.on_account_selected.call(account_id2.clone().into());
                                                main_form_state.write().hide_dialog();
                                             },
                                                class:"selectAccountId",
                                                td{colspan:"4",
                                                   "{account_id}"
                                                }

                                            }

                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
        };

        result.push(content);
    }

    render! {result.into_iter()}
}

#[inline_props]
fn render_profit(cx: Scope, selected: bool, currency: String) -> Element {
    if *selected {
        render! {
            div { style: "color:green", "{currency}0" }
            div { style: "color: var(--label-color)", "Profit" }
        }
    } else {
        render! { div {} }
    }
}

#[inline_props]
fn render_available(cx: Scope, selected: bool, currency: String) -> Element {
    if *selected {
        render! {
            div { "{currency}0" }
            div { style: "color: var(--label-color)", "Available" }
        }
    } else {
        render! { div {} }
    }
}

#[inline_props]
fn render_invested(cx: Scope, selected: bool, currency: String) -> Element {
    if *selected {
        render! {
            "{currency}0"
            div { style: "color: var(--label-color)", "Invested" }
        }
    } else {
        render! { div {} }
    }
}
