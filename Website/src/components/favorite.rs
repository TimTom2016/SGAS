use charming::{component::{Axis, Title}, element::AxisType, series::Line, Chart, WasmRenderer};
use itertools::Itertools;
use rand::Rng;
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
            let mut rng = rand::thread_rng();
            let chart = Chart::new()
                .title(Title::new().text("Demo: Leptos + Charming"))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data((0..7).map(|x| rng.gen_range(0..500 )).collect_vec()));

            let renderer = WasmRenderer::new(300, 300);
            renderer.render_element(node_ref.get().unwrap().deref().clone().into(),&chart).unwrap();
            }
    });
    create_effect(move |_| {
        set_timeout(move || {
			render_chart.dispatch(());
		}, Duration::from_secs(1));
    });
    view!{
        <div class="shadow card scroll-animation">
            <div class="d-flex flex-column card-body">
                <button class="m-2 btn btn-primary">Edit</button>
                <div class="d-flex align-items-center rounded justify-content-center flex-grow-1">
                    <div _ref=node_ref></div>
                </div>
        
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