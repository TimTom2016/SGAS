use leptos::*;

#[component]
pub fn NotLoggedInHomePage() -> impl IntoView {
    view!{
        <div class="d-flex flex-grow-1 justify-content-center align-items-center flex-column">
            <h1 class="text-center willkommen">Willkommen</h1>
            <h3 class="text-center willkommen-2"> Bitte loggen sie sich ein </h3>
        </div>
    }

}
