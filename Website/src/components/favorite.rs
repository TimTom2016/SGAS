use charming::{component::{Axis, Title}, element::AxisType, series::Line, Chart, WasmRenderer};
use std::{ops::Deref, time::Duration};
use html::Div;
use icondata::BiPlusCircleRegular;
use leptos::*;
use thaw::*;

#[component]
pub fn Favorite() -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let render_chart = create_action(move |_ : &()| {
        async move {
            let chart = Chart::new()
                .title(Title::new().text("Demo: Leptos + Charming"))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

            let renderer = WasmRenderer::new(600, 400);
            renderer.render_element(node_ref.get().unwrap().deref().clone().into(),&chart).unwrap();
            }
    });
    create_effect(move |_| {
        set_timeout(move || {
			render_chart.dispatch(());
		}, Duration::from_secs(1));
    });
    view!{
        <div class="d-flex flex-column min-vh-70 min-vw-70 ">
            <div class="d-flex flex-row ">
                <hr class="flex-fill align-self-center"/>
                <button class="m-2 btn btn-primary">Edit</button>
                <hr class="flex-fill align-self-center"/>
            </div>
            <div class="d-flex align-items-center rounded justify-content-center flex-grow-1">
                <div _ref=node_ref></div>
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