use dioxus::prelude::*;

use crate::views::icons::*;

pub fn left_panel(cx: Scope) -> Element {
    let element = use_state(cx, || false);

    render! {
        div { id: "leftPanel",
            div {
                style: "filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.3));",
                onclick: move |_| {
                    let value = *element.get();
                    element.set(!value);
                },
                markets_icon {}
            }
            div { class: "left-panel-label", "Markets" }
            div { class: "left-panel-separator" }
            div { style: "filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.3));", portfolio_icon {} }
            div { class: "left-panel-label", "Portfolio" }
            div { class: "left-panel-separator" }
            div { style: "filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.3));", history_icon {} }

            div { class: "left-panel-label", "History" }
            div { class: "left-panel-separator" }
            render_my_script { value: *element.get() }
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
