use dioxus::prelude::*;

#[inline_props]
pub fn render_avatar(cx: Scope, id: String) -> Element {
    render! {
        div { class: "instr-avatar", img { src: "/avatar/{id}", style: "width: 32px; height: 32px;" } }
    }
}
