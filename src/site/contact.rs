use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
    <>
        <Header />
        <div class="contact">
            <h1>{ "Contact Us" }</h1>
            <p>{ "This is the contact page of our Yew application." }</p>
        </div>
        <div class="contact-form">
            <form method="post">
                <label for="name">{ "Name:" }</label>
                <input type="text" id="name" name="name" />

                <label for="email">{ "Email:" }</label>
                <input type="email" id="email" name="email" />
                
                <label for="message">{ "Message:" }</label>
                <textarea id="message" name="message" rows="4" />

                <button type="submit">{ "Send Message" }</button>
            </form>
            
        </div>
        <style>
            { contact_style() }
        </style>
    </>
    }
}

fn contact_style() -> String {
    r#"
    .contact {
        padding: 2rem;
        text-align: center;
    }
    .contact h1 {
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }
    .contact p {
        font-size: 1.2rem;
        color: #555;
    }
    .contact-form {
        max-width: 600px;
        margin: 0 auto;
        padding: 1rem;
        background-color: #f9f9f9;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
    .contact-form form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
    .contact-form label {
        font-weight: bold;
        margin-bottom: 0.5rem;
    }
    .contact-form input, .contact-form textarea {
        width: 96%;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
    }
    .contact-form button {
        padding: 0.75rem;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 1rem;
    }
    .contact-form button:hover {
        background-color: #0056b3;
    }
    "#.to_string()
}

