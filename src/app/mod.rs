pub mod dashboard;
pub mod login;

use yew::prelude::*;
use yew_router::prelude::*;

use dashboard::Dashboard;
use login::Login;

#[derive(Routable, PartialEq, Clone)]
pub enum AppRoute {
    #[at("/")]
    Login,

    #[at("/dash")]
    Dashboard,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! { <Login /> },
        AppRoute::Dashboard => html! { <Dashboard /> },
        AppRoute::NotFound => html! { "Page Not Found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}