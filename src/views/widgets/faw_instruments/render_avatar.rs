use dioxus::prelude::*;

use crate::types::InstrumentId;

#[inline_props]
pub fn render_avatar(cx: Scope, id: InstrumentId) -> Element {
    let id = id.as_str();
    render! {
        div { class: "instr-avatar", img { src: "/avatar/{id}", style: "width: 32px; height: 32px;" } }
    }
}
