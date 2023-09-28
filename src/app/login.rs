use database_lib::database::{DatabaseConnection, bson::User};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use math_lib::{MathExpr, MathComponent::{Add, Sub, Mul, Power, Bracket, Num}};

use crate::components::inputs::{
    GradientBorderButton,
    GradientBorderInput,
};

#[function_component(Login)]
pub fn login() -> Html {
    let centered_div = classes!(
        "flex", "flex-column",
        "justify-content-center",
        "align-items-center",

        "background-color-bg",

        "padding-2rem",
    );

    let username = use_state(|| String::new());
    let password = use_state(|| String::new());

    let onchange_username = {
        let username = username.clone();
        move |e: Event| {
            let value = e.target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("Input Element Failed to cast")
                .value();

            username.set(value);
        }
    };

    let onchange_password = {
        let password = password.clone();
        move |e: Event| {
            let value = e.target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("Input Element Failed to cast")
                .value();

            password.set(value);
        }
    };

    let onclick = |_| {
        
    };

    html! {
        <div style={"background-image: url('public/slate.jpeg'); background-position: center;"} class={classes!("flex", "flex-column", "justify-content-center", "align-items-center", "height-100vh")}>
            <div class={centered_div}>
                <h1>{"Times Tables Blitz"}</h1>
                <GradientBorderInput class={classes!("margin-1hrem")} min_length={3} max_length={20} onchange={onchange_username} required={true} id={"username"}>{ "Username" }</GradientBorderInput>
                <p class={classes!("color-accent", "font-size-3qrem")}>{"3-20 characters*"}</p>
                <GradientBorderInput input_type={"password"} class={classes!("margin-1hrem")} min_length={8} max_length={20} onchange={onchange_password} required={true} id={"password"}>{ "Password" }</GradientBorderInput>
                <p class={classes!("color-accent", "font-size-3qrem")}>{"8-20 characters*"}</p>
                <GradientBorderButton {onclick}>{ "Sign In" }</GradientBorderButton>
            </div>
        </div>
    }
}