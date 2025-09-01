use yew::prelude::*;
use crate::site::header::Header;

#[function_component(About)]
pub fn about() -> Html {
    html! {
    <>
        <Header />
        <div class="about">
            <h1>{ "About Us" }</h1>
            <p>{ "This is the about page of our Yew application." }</p>
        </div>
    </>
    }
}