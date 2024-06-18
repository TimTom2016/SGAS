use std::time::Duration;

use ev::SubmitEvent;
use futures::TryFutureExt;
use itertools::Itertools;
use leptos::*;
use leptos_router::*;
use num_traits::{FromPrimitive, ToPrimitive};
use thaw::*;

use crate::shared::graph_types::GraphTypes;


#[component]
pub fn CreateFavorite(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let selected = create_rw_signal("Existing Graphs".to_string());
    view!{
        <Tabs value=selected>
            <Tab key="Existing Graphs">
                <Existing x y reload=reload/>
            </Tab>
            <Tab key="Create Graphs">
                <NewGraph x y reload=reload/>
            </Tab>
        </Tabs>
    }
}

#[component]
pub fn Existing(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let mut got_triggered = create_rw_signal(false);
    let on_submit = move |ev: SubmitEvent| {
        let data = CreateExistingFavorite::from_event(&ev);
        // silly example of validation: if the todo is "nope!", nope it
        if data.is_err() || data.unwrap().graph_id == -1 {
            // ev.prevent_default() will prevent form submission
            ev.prevent_default();
        }
        got_triggered.set(true);
    };
    let graphs = create_resource(|| (), |_| {
        async move {
            get_existing_graphs().await.unwrap()
        }
    });
    let action = create_server_action::<CreateExistingFavorite>();
    create_effect(move |_| {
        if action.pending().get() == false && got_triggered.get(){
            set_timeout(move || reload.with_value(|reload| reload())
                , Duration::from_millis(100));
        };
    });
    view!{
        <Suspense>
        <ActionForm action=action on:submit=on_submit>
            <input type="hidden" value=move || x name="x"/> 
            <input type="hidden" value=move || y name="y"/> 
            <div class="mb-3">
                <select class="form-select" aria-label="Select an graph" name="graph_id">
                    <option selected value=-1>Select an Existing Graph</option>
                    {move || graphs.get().map(|graphs| {
                        view!{
                        <For
                        each=move || graphs.clone()
                        key=move |data| data.1
                        let:data
                        > 
                            <option value=data.1> {data.0} </option>
                        </For>
                    }})}
                    
                </select>
            </div>
            <button type="submit" class="btn btn-primary">Submit</button>
        </ActionForm>
        </Suspense>
    }
}

#[server]
pub async fn get_existing_graphs() -> Result<Vec<(String,i32)>,ServerFnError> {
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    }
    let results = sqlx::query!("SELECT id,name FROM graph WHERE user_id = ?",user.unwrap().id).fetch_all(db.get_pool()).await.unwrap();
    Ok(results.into_iter().map(|value| (value.name,value.id)).collect_vec())
}

#[server]
pub async fn create_existing_favorite(graph_id: i32, x: u32, y:u32) -> Result<(),ServerFnError::<String>>{
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    };
    let user = user.unwrap();
    let graph = sqlx::query!("SELECT user_id FROM graph WHERE id=?",graph_id).fetch_one(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Graph not existing".to_string()))?;
    if user.id as i32 == graph.user_id {
        sqlx::query!("INSERT INTO favorites (graph, user_id, x, y) VALUES (?, ?,?,?)",graph_id,user.id,x,y).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not create favorite".to_string()))?;
        return Ok(())
    } else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }

}



#[component]
pub fn NewGraph(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let mut got_triggered = create_rw_signal(false);
    let on_submit = move |ev: SubmitEvent| {
        let data = CreateNewGraphAndFavorite::from_event(&ev);
        if data.is_err() || data.unwrap().graph_type == -1 {
            // ev.prevent_default() will prevent form submission
            ev.prevent_default();
        }
        got_triggered.set(true);
    };
    let sensors = create_resource(|| (), |_| {
        async move {
            get_existing_sensors().await.unwrap()
        }
    });
    let action = create_server_action::<CreateNewGraphAndFavorite>();
    create_effect(move |_| {
        if action.pending().get() == false && got_triggered.get(){
            set_timeout(move || reload.with_value(|reload| reload())
                , Duration::from_millis(100));
        };
    });
    view!{
        <Suspense>
        <ActionForm action=action on:submit=on_submit>
            <input type="hidden" value=move || x name="x"/> 
            <input type="hidden" value=move || y name="y"/> 
            <div class="mb-3">
                <label for="NameInput" class="form-label">Name</label>
                <input type="text" class="form-control" id="NameInput" aria-describedby="NameHelp" name="name"/>
                <div id="NameHelp" class="form-text">This is the Name that the Graph will have</div>
            </div>


            <div class="mb-3">
                <select class="form-select" aria-label="Select an Sensor" name="sensor_id">
                    <option selected value=-1>Select an Sensor</option>
                    {move || sensors.get().map(|graphs| {
                        view!{
                        <For
                        each=move || graphs.clone()
                        key=move |data| data.1
                        let:data
                        > 
                            <option value=data.1> {data.0} </option>
                        </For>
                    }})}
                    
                </select>
            </div>
            <div class="mb-3">
                <select class="form-select" aria-label="Select an Sensor" name="graph_type">
                    <option selected value=-1>Select a GraphType</option>
                    <For
                    each=move || [GraphTypes::BasicLine,GraphTypes::SmoothedLine,GraphTypes::BasicBar].into_iter().map(|value| (value.to_string(),value.to_i32())).collect_vec()
                    key=move |data| data.1
                    let:data
                    > 
                        <option value=data.1> {data.0} </option>
                    </For>                    
                </select>
            </div>
            <button type="submit" class="btn btn-primary">Submit</button>
        </ActionForm>
        </Suspense>
    }
}

#[server]
pub async fn get_existing_sensors() -> Result<Vec<(String,i32)>,ServerFnError> {
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    }
    let results = sqlx::query!("SELECT sensorId,name FROM sensor WHERE sensorId IN (SELECT sensorId FROM canSee WHERE id = ?)",user.unwrap().id).fetch_all(db.get_pool()).await.unwrap();
    Ok(results.into_iter().map(|value| (value.name,value.sensorId)).collect_vec())
}



#[server]
pub async fn create_new_graph_and_favorite(name: String, sensor_id: i32,graph_type: i32, x: u32, y:u32) -> Result<(),ServerFnError::<String>>{
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    };
    if GraphTypes::from_i32(graph_type).is_none() {
        return Err(ServerFnError::ServerError("Invalid GraphType".to_string()));
    }
    let user = user.unwrap();
    let sensor = sqlx::query!("SELECT id FROM canSee WHERE sensorId=?",sensor_id).fetch_one(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Graph not existing".to_string()))?;
    if user.id as i32 == sensor.id {
        let id = sqlx::query!("INSERT INTO graph (name, sensor_id,graph_type, user_id) VALUES (?, ?,?,?)",name,sensor_id,graph_type,user.id).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not create graph".to_string()))?.last_insert_id();
        sqlx::query!("INSERT INTO favorites (graph, user_id, x, y) VALUES (?, ?,?,?)",id,user.id,x,y).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not create favorite".to_string()))?;
        return Ok(())
    } else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }

}
