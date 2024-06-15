use std::time::Duration;

use ev::SubmitEvent;
use futures::TryFutureExt;
use itertools::Itertools;
use leptos::*;
use leptos_router::*;
use num_traits::{FromPrimitive, ToPrimitive};
use thaw::*;

use crate::{components::create_favorite::get_existing_graphs, shared::graph_types::GraphTypes};
use icondata::BiTrashAltRegular;
#[component]
pub fn EditFavorite(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let selected = create_rw_signal("Edit Graph".to_string());
    view!{
        <Tabs value=selected>
            <Tab key="Edit Graph">
                <EditGraph x y reload/>
            </Tab>
            <Tab key="Delete Graph">
                <DeleteGraph x y reload/>
            </Tab>
        </Tabs>
    }
}


#[component]
pub fn EditGraph(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let mut got_triggered = create_rw_signal(false);
    let on_submit = move |ev: SubmitEvent| {
        let data = ChangeFavorite::from_event(&ev);
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
    let action = create_server_action::<ChangeFavorite>();
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
                <select class="form-select" aria-label="Change graph" name="graph_id">
                    <option selected value=-1>Select another Graph</option>
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
pub async fn change_favorite(graph_id: i32, x: u32, y:u32) -> Result<(),ServerFnError::<String>>{
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    };
    let user = user.unwrap();
    let favorite = sqlx::query!("SELECT user_id FROM favorites WHERE x=? AND y=?",x,y).fetch_one(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Graph not existing".to_string()))?;
    if user.id as i32 == favorite.user_id {
        sqlx::query!("UPDATE favorites SET graph = ? WHERE x = ? AND y = ?",graph_id,x,y).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not create favorite".to_string()))?;
        return Ok(())
    } else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }

}


#[component]
pub fn DeleteGraph(
    x: u32,
    y: u32,
    reload: StoredValue<impl Fn() + 'static>
) -> impl IntoView {
    let mut got_triggered = create_rw_signal(false);
    let on_submit = move |ev: SubmitEvent| {
        let data = DeleteFavorite::from_event(&ev);
        // silly example of validation: if the todo is "nope!", nope it
        if data.is_err() {
            // ev.prevent_default() will prevent form submission
            ev.prevent_default();
        }
        got_triggered.set(true);
    };
    let action = create_server_action::<DeleteFavorite>();
    create_effect(move |_| {
        if action.pending().get() == false && got_triggered.get(){
            set_timeout(move || reload.with_value(|reload| reload())
                , Duration::from_millis(100));
        };
    });
    view!{
        <ActionForm action=action on:submit=on_submit>
            <input type="hidden" value=move || x name="x"/> 
            <input type="hidden" value=move || y name="y"/> 

            // <div class="form-check mb-3">
            //     <input class="form-check-input" type="checkbox" value="true" id="flexCheckDefault" name="sure"/>
            //     <label class="form-check-label" for="flexCheckDefault">
            //     Are you Sure?
            //     </label>
            // </div>
            <button type="submit" class="btn btn-primary"><Icon icon=BiTrashAltRegular width="2em" height="2em"/></button>
        </ActionForm>
    }
}

#[server]
pub async fn delete_favorite(x: u32, y: u32) -> Result<(),ServerFnError::<String>>{
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    };
    let user = user.unwrap();
    let favorite = sqlx::query!("SELECT graph,user_id FROM favorites WHERE x=? AND y=?",x,y).fetch_one(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Graph not existing".to_string()))?;
    if user.id as i32 == favorite.user_id {
        let count = sqlx::query!("SELECT COUNT(*) as \"count\" FROM favorites WHERE graph=?",favorite.graph).fetch_one(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not get count of favorites".to_string()))?.count;
        if count > 1 {
            sqlx::query!("DELETE FROM favorites WHERE x=? AND y=?",x,y).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not delete Favorite".to_string()))?;
        } else {
            sqlx::query!("DELETE FROM favorites WHERE x=? AND y=?",x,y).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not delete Favorite".to_string()))?;
            sqlx::query!("DELETE FROM graph WHERE id=?",favorite.graph).execute(db.get_pool()).await.map_err(|e| ServerFnError::ServerError("Could not delete Graph".to_string()))?;

        }
        return Ok(())
    } else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }

}