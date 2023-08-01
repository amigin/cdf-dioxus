use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::icons::io_icons::*;
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

#[inline_props]
pub fn markets_icon<'s>(cx: Scope<'s>, class: &'s str) -> Element<'s> {
    cx.render(rsx! { Icon { width: 24, height: 24, class: class, icon: IoBarChartOutline } })
}

#[inline_props]
pub fn portfolio_icon<'s>(cx: Scope<'s>, class: &'s str) -> Element<'s> {
    cx.render(rsx! { Icon { width: 24, height: 24, class: class, icon: IoBriefcaseOutline } })
}
#[inline_props]
pub fn history_icon<'s>(cx: Scope<'s>, class: &'s str) -> Element<'s> {
    cx.render(rsx! { Icon { width: 24, height: 24, class: class, icon: IoReceiptOutline } })
}

pub fn instrument_search_icon(cx: Scope) -> Element {
    render! { Icon { width: 16, height: 16, fill: "var(--label-color)", icon: BsSearch } }
}

pub fn profile_menu_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 64, height: 64, fill: "black", icon: BsPersonBadge } })
}

pub fn close_settings_panel_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 24, height: 24, fill: "var(--label-color)", icon: BsX } })
}

pub fn calendar_settings_panel_icon(cx: Scope) -> Element {
    cx.render(
        rsx! {Icon { width: 16, height: 16, fill: "var(--label-color)", icon: BsCalendarEvent }},
    )
}

pub fn drop_down_icon(cx: Scope) -> Element {
    cx.render(
        rsx! { Icon { width: 8, height: 8, fill: "var(--label-color)", icon: BsCaretDownFill } },
    )
}
