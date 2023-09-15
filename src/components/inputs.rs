use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlInputElement, console};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GradientBorderButtonProps {
    #[prop_or(Callback::from(|_| ()))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Classes::new())]
    pub class: Classes,
    pub children: Children,
}

#[function_component(GradientBorderButton)]
pub fn gradient_border_button(props: &GradientBorderButtonProps) -> Html {
    let onclick = props.onclick.clone();
    html! {
        <div class={classes!("main-btn-border", props.class.clone())} {onclick}>
            <button class={classes!("main-btn")}>{ props.children.clone() }</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GradientBorderInputProps {
    #[prop_or(Callback::from(|_| ()))]
    pub onchange: Callback<Event>,
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    pub children: Children,
    pub id: AttrValue,
    pub required: bool,
    pub min_length: usize,
    pub max_length: usize,
    #[prop_or(Classes::new())]
    pub class: Classes,
}

#[function_component(GradientBorderInput)]
pub fn gradient_border_input(props: &GradientBorderInputProps) -> Html {
    let min_length = props.min_length.clone();
    let max_length = props.max_length.clone();

    let onfocusout = move |e: FocusEvent| {
        let ele = e.target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let ele = match ele {
            Some(ele) => ele,
            None => return,
        };
        let length = ele.value().len();
        if length < min_length || length > max_length {
            ele.set_attribute("valid", "false").expect("error setting valid status");
        } else {
            ele.set_attribute("valid", "true").expect("error setting valid status");
        }
        if length == 0 {
            ele.set_attribute("empty", "true").expect("error setting empty status");
        } else {
            ele.set_attribute("empty", "false").expect("error setting empty status");
        }
    };

    html! {
        <div class={classes!("main-input-border", props.class.clone())}>
            <label for={props.id.clone()} class={classes!("main-input-label")}>{ props.children.clone() }</label>
            <input type={props.input_type.clone()} name={props.id.clone()} id={props.id.clone()} required={props.required} class={classes!("main-input")} onchange={props.onchange.clone()} {onfocusout}/>
        </div>
    }
}