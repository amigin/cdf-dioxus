use dioxus::prelude::*;

use crate::views::{icons::*, main_form::left_panel::*};

use super::LeftPanelState;

#[derive(Props)]
pub struct RenderLeftPanelData<'s> {
    pub state: LeftPanelState,
    pub on_close: EventHandler<'s, ()>,
}

pub fn render_left_panel_data<'s>(cx: Scope<'s, RenderLeftPanelData<'s>>) -> Element<'s> {
    let title = match cx.props.state {
        LeftPanelState::Nothing => "",
        LeftPanelState::ShowMarkets => "Markets",
        LeftPanelState::ShowPortfolio => "Portfolio",
        LeftPanelState::ShowHistory => "History",
    };

    render! {
        div { id: "left-panel-data",
            div { class: "header",
                div { class: "title", "{title}" }
                div {
                    class: "close",
                    onclick: move |_| {
                        cx.props.on_close.call(());
                    },
                    close_icon_big {}
                }
            }
            left_panel_instruments {}
        }
    }
}
