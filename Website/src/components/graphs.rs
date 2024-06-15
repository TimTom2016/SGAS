use std::{ops::Deref, time::Duration};

use chrono::Local;
use html::Div;
use itertools::Itertools;
use leptos::*;
use charming::{component::{Axis, Title}, element::AxisType, series::{Bar, Line}, Chart, WasmRenderer};
use serde::{Deserialize, Serialize};

use crate::shared::graph_types::GraphTypes;
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct GraphData {
    pub x: chrono::DateTime<Local>,
    pub y: f32,
}


#[component]
pub fn Graph(
    r#type: GraphTypes,
    #[prop(into)]
    data: MaybeSignal<Vec<GraphData>>,
    ) -> impl IntoView {
    let data_2 = data.clone();
    create_effect(move |_| {
        println!("{:?}",data_2.get())
    });
    view!{
        <div>
            { match r#type {
                GraphTypes::BasicLine => view!{<BasicLine data=data/>},
                GraphTypes::SmoothedLine => view!{<SmoothedLine data=data/>},
                GraphTypes::BasicBar => view!{<BasicBar data=data/>},
            }}
        </div>
    }
}



#[component]
pub fn BasicLine(
    #[prop(into)]
    data: MaybeSignal<Vec<GraphData>>) -> impl IntoView {
    let echarts = create_rw_signal(None);
    let node_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        let chart = Chart::new()
                    .title(Title::new().text(""))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(data.get().into_iter().map(|x| x.x.format("%H:%M:%S").to_string()).collect_vec()),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .animation(false)
                    .series(Line::new().data(data.get().into_iter().map(|x| x.y.to_string()).collect_vec()));
    
        let renderer = WasmRenderer::new(300, 300);
        if echarts.get().is_none() {
            echarts.set(Some(renderer.render_element(node_ref.get().unwrap().deref().clone().into(),&chart).unwrap()));

        } else {
            WasmRenderer::update(&echarts.get().unwrap(),&chart);
        }
    });
    view!{
        <div _ref=node_ref/>
    }

} 


#[component]
pub fn SmoothedLine(
    #[prop(into)]
    data: MaybeSignal<Vec<GraphData>>) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let echarts = create_rw_signal(None);
    create_effect(move |_| {
        let chart = Chart::new()
                    .title(Title::new().text(""))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(data.get().into_iter().map(|x| format!("{}",x.x.format("%H:%M:%S"))).collect_vec()),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .animation(false)
                    .series(Line::new().data(data.get().into_iter().map(|x| format!("{}",x.y)).collect_vec()).smooth(true));
    
        let renderer = WasmRenderer::new(300, 300);
        if echarts.get().is_none() {
            echarts.set(Some(renderer.render_element(node_ref.get().unwrap().deref().clone().into(),&chart).unwrap()));

        } else {
            WasmRenderer::update(&echarts.get().unwrap(),&chart);
        }
    });
    view!{
        <div _ref=node_ref/>
    }
    
} 


#[component]
pub fn BasicBar(
    #[prop(into)]
    data: MaybeSignal<Vec<GraphData>>) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let echarts = create_rw_signal(None);
    create_effect(move |_| {
        let chart = Chart::new()
            .title(Title::new().text(""))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(data.get().into_iter().map(|x| x.x.format("%H:%M:%S").to_string()).collect_vec()),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .animation(false)
            .series(Bar::new().data(data.get().into_iter().map(|x| x.y.to_string()).collect_vec()));

        let renderer = WasmRenderer::new(300, 300);
        if echarts.get().is_none() {
            echarts.set(Some(renderer.render_element(node_ref.get().unwrap().deref().clone().into(),&chart).unwrap()));

        } else {
            WasmRenderer::update(&echarts.get().unwrap(),&chart);
        }
    });
    view!{
        <div _ref=node_ref/>
    }
    
} 
