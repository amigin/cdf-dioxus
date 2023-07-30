use dioxus::prelude::*;

pub fn change_password_frame(cx: Scope) -> Element {
    render! {
        div {
            label { class: "form-label", "Current password" }
            input {
                class: "form-control",
                r#type: "password",
                placeholder: "Type current password here"
            }
        }

        div {
            label { class: "form-label", "New password" }
            input { class: "form-control", r#type: "password", placeholder: "Type new password here" }
        }

        div {
            label { class: "form-label", "New password again" }
            input {
                class: "form-control",
                r#type: "password",
                placeholder: "Type new password again here"
            }
        }

        div { style: "margin-top:20px", button { class: "btn btn-success", "Change password" } }
    }
}
