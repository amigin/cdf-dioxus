use dioxus::prelude::*;

use crate::views::{icons::*, main_form::left_panel::LeftPanelState};

pub fn left_panel(cx: Scope) -> Element {
    let left_panel_state = use_state(cx, || LeftPanelState::new());

    let mut markets_class = "left-panel-icon";
    let mut portfolio_class = "left-panel-icon";
    let mut history_class = "left-panel-icon";

    match left_panel_state.get() {
        LeftPanelState::ShowMarkets => markets_class = "left-panel-icon-active",
        LeftPanelState::ShowPortfolio => portfolio_class = "left-panel-icon-active",
        LeftPanelState::ShowHistory => history_class = "left-panel-icon-active",
        LeftPanelState::Nothing => {}
    }

    render! {
        div { id: "leftPanel",
            div {

                onclick: move |_| {
                    let value = left_panel_state.get();
                    left_panel_state.set(value.toggle_show_markets());
                },
                markets_icon { class: markets_class }
            }
            div { class: "left-panel-label", "Markets" }
            div { class: "left-panel-separator" }
            div {

                onclick: move |_| {
                    let value = left_panel_state.get();
                    left_panel_state.set(value.toggle_show_portfolio());
                },
                portfolio_icon { class: portfolio_class }
            }
            div { class: "left-panel-label", "Portfolio" }
            div { class: "left-panel-separator" }
            div {
                onclick: move |_| {
                    let value = left_panel_state.get();
                    left_panel_state.set(value.toggle_show_history());
                },
                history_icon { class: history_class }
            }

            div { class: "left-panel-label", "History" }
            div { class: "left-panel-separator" }
            render_my_script { value: left_panel_state.get().is_panel_shown() }
        }
    }
}

#[inline_props]
fn render_my_script(cx: Scope, value: bool) -> Element {
    if *value {
        render! { script { r#"show_panel()"# } }
    } else {
        render! { script { r#"hide_panel()"# } }
    }
}
