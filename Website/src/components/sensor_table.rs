use std::collections::VecDeque;
use std::ops::Range;
use leptos::*;
use leptos_router::{A, Url};
use leptos_struct_table::*;
use leptos_use::use_debounce_fn_with_arg;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow,QueryBuilder};
use crate::components::paginator::Paginator;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SensorQuery {
	#[serde(default)]
	sort: VecDeque<(usize, ColumnSort)>,
	range: Range<usize>,
	search: String,
}

pub struct SensorDataProvider {
	sorting: VecDeque<(usize, ColumnSort)>,
	pub search: RwSignal<String>,
}

#[derive(TableRow, Clone,Serialize,Deserialize)]
#[cfg_attr(all(feature="ssr"),derive(FromRow))]
#[table(sortable,classes_provider = "BootstrapClassesPreset",)]
pub struct Sensor {
	#[table(skip)]
    #[cfg_attr(all(feature="ssr"),sqlx(rename="sensorId"))]
	pub id: i32,
	#[table(title="Name")]
	pub name: String,
	#[table(title="Type")]
    #[cfg_attr(all(feature="ssr"),sqlx(rename="type"))]
	pub sensor_type: String,
	#[table(title="Pin")]
	pub pin: Option<i32>,
    #[table(title="I²C Addr")]
	pub addr: Option<String>,

}


impl Default for SensorDataProvider {
	fn default() -> Self {
		Self {
			sorting: VecDeque::new(),
			search: RwSignal::new("".to_string()),
		}
	}
}
impl PaginatedTableDataProvider<Sensor> for SensorDataProvider {
	const PAGE_ROW_COUNT: usize = 50;

	async fn get_page(&self, page_index: usize) -> Result<Vec<Sensor>, String> {
		match list_persons(SensorQuery {sort: self.sorting.clone(),search: self.search.get().to_string(), range: page_index*Self::PAGE_ROW_COUNT..(((page_index+1)*Self::PAGE_ROW_COUNT)-1)}).await {
			Ok(data) => Ok(data),
			Err(err) => Err(err.to_string()),
		}
	}
	async fn row_count(&self) -> Option<usize> {
		match get_row_count().await {
			Ok(rows) => Some(rows),
			Err(_) => None,
		}
	}

	fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
		self.sorting = sorting.clone();
	}

	fn track(&self) {
		self.search.track();
	}

}


#[component]
pub fn SensorTable() -> impl IntoView {
	let rows = SensorDataProvider::default();
	//let search = rows.search;
	//let on_input = use_debounce_fn_with_arg(move |value| rows.search.set(value), 300.0);
	let pagination_controller = PaginationController::default();
	return view! {
        <table class="table-striped table table-hover">
            <TableContent rows 
			display_strategy=DisplayStrategy::Pagination {
            controller: pagination_controller,
            row_count: 10
            }
			/>
        </table>
		<Paginator pagination_controller />
    };
}

#[server]
pub async fn list_persons(query: SensorQuery) -> Result<Vec<Sensor>, ServerFnError<String>> {
	use sqlx::FromRow;
	use sqlx::QueryBuilder;
    use crate::shared::app_state::AppState;
    use crate::auth::get_user;
	let user = get_user().await.unwrap();
    let db: crate::db::database::Database = expect_context::<AppState>().db;
    if user.is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()))
    };
    let user = user.unwrap();
    let SensorQuery { sort, range, search } = query;
	let mut query = QueryBuilder::new("SELECT * FROM sensor ");
	if !search.is_empty() {
        query.push(" WHERE name LIKE ");
        query.push("concat('%', ");
        query.push_bind(&search);
        query.push(", '%')");
        query.push(" AND sensorId IN (SELECT sensorId_id FROM canSee WHERE id=");
    } else {
        query.push(" WHERE sensorId IN (SELECT sensorId FROM canSee WHERE id=");
    }
    query.push_bind(user.id);
    query.push(")");
    
    if let Some(order) = Sensor::sorting_to_sql(&sort) {
        query.push(" ");
        query.push(order);
    }
    
    query.push(" LIMIT ");
    query.push_bind(range.len() as i64);
    query.push(" OFFSET ");
    query.push_bind(range.start as i64);
	let mut rows = query
		.build_query_as::<Sensor>()
		.fetch_all(db.get_pool())
		.await
		.map_err(|e| ServerFnError::WrappedServerError(format!("{e:?}")))?;
	Ok(rows)
}

#[server]
pub async fn get_row_count() -> Result<usize,ServerFnError<String>>
{
	use sqlx::{FromRow,query};
	use crate::shared::app_state::AppState;
	
	let db = use_context::<AppState>().unwrap().db;
	Ok(query!("SELECT COUNT(sensorId) as count FROM sensor").fetch_one(db.get_pool()).await.unwrap().count as usize)
}