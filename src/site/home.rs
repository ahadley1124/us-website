use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <>
        <div class="home-container">
            <Header />
        </div>
        <div class="home">
            <h1>{ "Welcome to the Home Page" }</h1>
            <p>{ "This is the home page of our Yew application." }</p>
            <p>{ "Feel free to explore the site!" }</p>
            <p>{ "You can navigate to other pages using the links in the navigation bar." }</p>
            <p>{ "Enjoy your stay!" }</p>
            <p>{ "If you have any questions, feel free to reach out." }</p>
            <p>{ "Thank you for visiting!" }</p>
        </div>
    </>
    }
}
