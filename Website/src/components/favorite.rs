use icondata::BiPlusCircleRegular;
use leptos::*;
use thaw::*;

#[component]
pub fn Favorite() -> impl IntoView {
    view!{
        <div class="d-flex flex-column min-vh-70 min-vw-70 ">
            <div class="d-flex flex-row ">
                <hr class="flex-fill align-self-center"/>
                <button class="m-2 btn btn-primary">Edit</button>
                <hr class="flex-fill align-self-center"/>
            </div>
            <div class="d-flex align-items-center bg-secondary rounded justify-content-center flex-grow-1" style="height: 500px;">
                <h1 class="text-center"> Content</h1>
            </div>
            <div class="mb-5">
                <hr class=""/>
            </div>
        </div>
    }
}

#[component]
pub fn AddFavorite() -> impl IntoView {
    view!{
        <div class="d-flex justify-content-center">
            <button class="btn btn-primary">
                <Icon icon=BiPlusCircleRegular width="2em" height="2em"/>
            </button> 
        </div>
    }
}