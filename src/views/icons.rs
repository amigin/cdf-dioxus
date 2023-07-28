use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::*;

pub fn person_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "black", icon: BsPersonBadge } })
}

pub fn close_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "var(--label-color)", icon: BsX } })
}

pub fn add_instrument_icon(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            width: 16,
            height: 16,
            fill: "var(--label-color)",
            icon: BsPlusSquareDotted
        }
    })
}
