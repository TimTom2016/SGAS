use std::time::Duration;

use icondata::{BiPlusCircleRegular,BiEditAltRegular};
use leptos::*;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use thaw::*;
use crate::{auth::get_user, components::{graphs::*,create_favorite::{CreateFavorite},edit_favorite}, shared::graph_types::GraphTypes};

#[component]
pub fn Favorite(
    #[prop(into)]
    id: MaybeSignal<u32>,
    #[prop(into)]
    x: MaybeSignal<u32>,
    #[prop(into)]
    y: MaybeSignal<u32>,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let id = create_rw_signal(id.get());
    let definition = create_resource(id, |id| async move {
        get_data(id).await.unwrap()
    });
    
    create_effect(move |_| {
        set_interval(move || {
            definition.refetch()
        }, Duration::from_secs(1))
    });
    view!{
        <div class="shadow card scroll-animation">
            <div class="d-flex flex-column card-body">
                <EditFavorite x=x.get() y=y.get() reload/>
                
                <div class="d-flex align-items-center rounded justify-content-center flex-grow-1 flex-column">
                    <Transition>
                    {move || definition.get().map(|definition| view!{
                        <p class="text-center">{definition.title}</p>
                        <Graph r#type=definition.graph_type data=definition.data/>
                    })
                    }
                    </Transition>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EmptyFavorite(
    #[prop(into)]
    x: MaybeSignal<u32>,
    #[prop(into)]
    y: MaybeSignal<u32>,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    view!{
        <div class="shadow card scroll-animation">
            <div class="d-flex flex-column card-body">
                <div class="d-flex align-items-center rounded justify-content-center flex-grow-1" style:width="300px" style:height="300px">
                    <AddFavorite x= x.get() y=y.get() reload/>
                </div>
        
            </div>
        </div>
    }
}

#[component]
pub fn EditFavorite(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let show = create_rw_signal(false);
    view!{
        <div class="d-flex justify-content-center">
            <button class="btn btn-primary" on:click=move |_| show.set(true)>
                <Icon icon=BiEditAltRegular width="1em" height="1em"/>
            </button> 
        </div>
        <Modal show>
            <div class="d-flex justify-content-center align-items-center">
                <edit_favorite::EditFavorite x y reload/>
            </div>
        </Modal>
    }
}


#[component]
pub fn AddFavorite(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let show = create_rw_signal(false);
    view!{
        <div class="d-flex justify-content-center">
            <button class="btn btn-primary" on:click=move |_| show.set(true)>
                <Icon icon=BiPlusCircleRegular width="2em" height="2em"/>
            </button> 
        </div>
        <Modal show>
            <div class="d-flex justify-content-center align-items-center">
                <CreateFavorite x y reload/>
            </div>
        </Modal>
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub struct GraphDefinition {
    pub title: String,
    pub graph_type: GraphTypes,
    pub data: Vec<GraphData>,
}

#[server]
pub async fn get_data(id: u32) -> Result<GraphDefinition,ServerFnError::<String>> {
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
	let graph = sqlx::query!("SELECT * FROM graph WHERE id=?",id).fetch_one(db.get_pool()).await.unwrap();
    let can_see = sqlx::query!("SELECT * FROM canSee WHERE id=?",user.unwrap().id).fetch_optional(db.get_pool()).await.unwrap();
    if can_see.is_none() {
        return Err::<GraphDefinition,ServerFnError::<String>>(ServerFnError::ServerError::<String>("Not Authenticated".to_string()));
    }
    let mut values = sqlx::query!("SELECT * FROM sensorValue WHERE sensorId_id=? ORDER BY time_stamp DESC LIMIT 100",graph.sensor_id).fetch_all(db.get_pool()).await.unwrap();
    values.reverse();
    let mut return_values = vec![];
    for value in values {
        return_values.push(GraphData {
            x: value.time_stamp.with_timezone(&chrono::Local),
            y: value.value
        });
    }
    return Ok(GraphDefinition{
        title: graph.name,
        graph_type: GraphTypes::from_i32(graph.graph_type).unwrap(),
        data: return_values,
    });
}