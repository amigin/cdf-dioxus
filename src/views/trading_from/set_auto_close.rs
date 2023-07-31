use dioxus::prelude::*;

use crate::{states::*, views::icons::*, MOCK_CURRENCY_USD};

pub fn render_set_auto_close(cx: Scope) -> Element {
    let trading_panel_state = use_shared_state::<TradingPanelState>(cx).unwrap();

    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let instrument_id = fav_instruments_state.read().get_selected().clone();

    let (tp_mode, sl_mode) = {
        let read_access = trading_panel_state.read();
        (read_access.profit_value_mode, read_access.loss_value_mode)
    };

    let tp_sl = trading_panel_state.read().get_tp_or_sl(&instrument_id);

    let (tp_value, sl_value) = if let Some((tp, sl)) = &tp_sl {
        let tp_value = if let Some(tp_value) = &tp {
            use_state(cx, || tp_value.clone())
        } else {
            use_state(cx, || tp_mode.into_value("".to_string()))
        };

        let sl_value = if let Some(sl_value) = &sl {
            use_state(cx, || sl_value.clone())
        } else {
            use_state(cx, || tp_mode.into_value("".to_string()))
        };

        (tp_value, sl_value)
    } else {
        let tp_value = use_state(cx, || tp_mode.into_value("".to_string()));
        let sl_value = use_state(cx, || sl_mode.into_value("".to_string()));

        (tp_value, sl_value)
    };

    let mut result = Vec::with_capacity(2);

    result.push(if let Some((tp, sl)) = &tp_sl {
        let tp = if let Some(tp) = tp {
            format!("T/P: {}", tp.to_panel_string(MOCK_CURRENCY_USD))
        } else {
            "".to_string()
        };

        let sl = if let Some(sl) = sl {
            format!("S/L: {}", sl.to_panel_string(MOCK_CURRENCY_USD))
        } else {
            "".to_string()
        };

        rsx! {
            button {
                id: "btnAutoClose",
                class: "btn btn-outline-dark",
                onclick: move |_| {
                    trading_panel_state.write().toggle_set_auto_close();
                },
                format!("{} {}",tp, sl).trim()
            }
        }
    } else {
        rsx! {
            button {
                id: "btnAutoClose",
                onclick: move |_| {
                    trading_panel_state.write().toggle_set_auto_close();
                },
                class: "btn btn-outline-dark",
                "Set"
            }
        }
    });

    if trading_panel_state.read().is_show_set_auto_close() {
        result.push(rsx! {
            div { id: "setAutoClosePanel",
                div { class: "floating-panel-header",
                    div { class: "title", "Set Auto close" }
                    div {
                        class: "close-icon",
                        onclick: move |_| {
                            trading_panel_state.write().hide_everything();
                        },
                        close_icon {}
                    }
                }

                div { style: "margin-top: 15px",
                    label { "When profit is:" }
                    div { class: "btn-group",
                        span {
                            class: "input-group-text input-group-text-sm",
                            style: "cursor: pointer",
                            onclick: move |_| {
                                trading_panel_state.write().toggle_tp_mode();
                            },
                            tp_mode.get_sign()
                        }
                        input {
                            class: "form-control form-control-sm",
                            value: tp_value.as_str(),
                            oninput: move |e| {
                                tp_value.set(tp_value.get().update(e.value.clone()));
                            }
                        }
                    }
                }

                div { style: "margin-top: 15px",
                    label { "When loss is:" }
                    div { class: "btn-group",
                        span {
                            class: "input-group-text input-group-text-sm",
                            style: "cursor: pointer",
                            onclick: move |_| {
                                trading_panel_state.write().toggle_sl_mode();
                            },
                            sl_mode.get_sign()
                        }
                        input {
                            class: "form-control form-control-sm",
                            value: sl_value.as_str(),
                            oninput: move |e| {
                                sl_value.set(sl_value.get().update(e.value.clone()));
                            }
                        }
                    }
                }

                div { style: "margin-top: 20px",
                    button {
                        class: "btn btn-success btn-sm",

                        onclick: move |_| {
                            let tp = tp_value.get().as_str();
                            let sl = sl_value.get().as_str();
                            let tp = tp.trim();
                            let sl = sl.trim();
                            let mut write = trading_panel_state.write();
                            if tp.len() == 0 {
                                write.set_tp(&instrument_id, None);
                            } else {
                                write.set_tp(&instrument_id, Some(tp_mode.into_value(tp.to_string())));
                            }
                            if sl.len() == 0 {
                                write.set_sl(&instrument_id, None);
                            } else {
                                write.set_sl(&instrument_id, Some(sl_mode.into_value(sl.to_string())));
                            }
                            write.hide_everything();
                        },
                        style: "width:100%",
                        "Apply"
                    }
                }
            }
        });
    }

    render! {result.into_iter()}
}
