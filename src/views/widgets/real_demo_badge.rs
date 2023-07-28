use dioxus::prelude::*;

#[inline_props]
pub fn real_demo_badge(cx: Scope, is_live: bool) -> Element {
    let (acc_type_class, acc_type_name) = if *is_live {
        ("badge text-bg-success acc_type-badge-live", "Real")
    } else {
        ("badge text-bg-light acc_type-badge-demo", "Demo")
    };

    render! { span { class: "{acc_type_class}", "{acc_type_name}" } }
}
