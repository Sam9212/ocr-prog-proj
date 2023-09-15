use yew::prelude::*;

use crate::components::info_bar::AccountInfoBar;

#[derive(PartialEq, Properties)]
pub struct DashboardProps {}

#[function_component(Dashboard)]
pub fn dashboard(_props: &DashboardProps) -> Html {
    html! {
        <div class={classes!(vec!["flex", "flex-column-reverse"])}>
            <AccountInfoBar />
        </div>
    }
}