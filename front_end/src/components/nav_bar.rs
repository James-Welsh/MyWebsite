use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavigationBar {
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub nav_bar: NavigationBar,
}

#[function_component(NavBar)]
pub fn nav_bar(NavBarProps { nav_bar }: &NavBarProps) -> Html {
    html! {
        <div>
            <h1>{&nav_bar.title}</h1>
        </div>
    }
}