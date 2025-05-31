use yew::prelude::*;
use yew_router::prelude::*;
use yew::classes;
use crate::Route; // Make sure this path matches where your Route enum is defined

#[function_component(Header)]
pub fn header() -> Html {
    html! {
    <>
        <style>
            { header_style() }
        </style>
        <header class={classes!("header")}>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::About}>{ "About" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                </ul>
            </nav>
        </header>
    </>
    }
}

/// Returns a string containing CSS styles for the header.
fn header_style() -> String {
    r#"
    .header {
        background-color: #222;
        color: #fff;
        padding: 1rem 0;
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
    .header nav ul {
        list-style: none;
        display: flex;
        gap: 2rem;
        margin: 0;
        padding: 0;
        justify-content: center;
    }
    .header nav ul li {
        display: inline;
    }
    .header nav ul li a {
        color: #fff;
        text-decoration: none;
        font-weight: 500;
        transition: color 0.2s;
    }
    .header nav ul li a:hover {
        color: #ffd700;
    }
    "#.to_string()
}
