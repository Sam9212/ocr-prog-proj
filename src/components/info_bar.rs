use yew::prelude::*;

#[function_component(AccountInfoBar)]
pub fn account_info_bar() -> Html {
    html! {
        <div class={classes!(vec!["flex", "flex-row"])}>
            <p>{ "Signed in as Sam9212" }</p>
        </div>
    }
}