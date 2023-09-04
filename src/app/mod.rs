pub mod home;
pub mod login;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use login::Login;

use crate::components::header::Header;

#[derive(Routable, PartialEq, Clone)]
pub enum AppRoute {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::Login => html! {<Login />},
        AppRoute::NotFound => html! { "Page Not Found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Header />
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}