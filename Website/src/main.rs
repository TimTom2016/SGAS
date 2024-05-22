#![feature(associated_type_defaults)]
mod shared;
mod pages;
mod components;

pub use cfg_if::cfg_if;
use dotenvy::var;

cfg_if! { if #[cfg(feature = "ssr")] {
    mod auth;
    use crate::db::repositories::user::UserRepository;
    use crate::db::database::Database;
    use sqlx::MySqlPool;
    use axum::response::Response as AxumResponse;
    use http::HeaderMap;
    use axum::{extract::{Path, Request, State}, response::IntoResponse};
    use leptos_axum::handle_server_fns_with_context;
    use axum::{routing::{post,get}, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use pages::main::App;
    use tokio::net::TcpListener;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    pub use axum_session_sqlx::SessionMySqlPool;
    use crate::shared::user::User;
    pub mod domain;
    use crate::shared::app_state::AppState;
    pub mod db;
    pub mod fileserv;
    use crate::auth::ssr::AuthSession;
    use dotenvy::dotenv;
    async fn leptos_routes_handler(State(app_state): State<AppState>,
    auth_session: AuthSession, req: Request) -> AxumResponse {
        let handler = leptos_axum::render_route_with_context(
            app_state.options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.clone());
                provide_context(app_state.options.clone());
            },
            || view! {<App/>},
        );
        handler(req).await.into_response()
}

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        _path: Path<String>,
        _headers: HeaderMap,
        _query: axum::extract::RawQuery,
        request: Request,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.clone());

            },
            request,
        )
        .await
    }
    #[tokio::main]
    pub async fn main() {
        dotenv().expect("couldn't load .env");
        let database_url = match var("DATABASE_URL")  {
            Ok(var) => var,
            Err(_) => return
        };
        leptos_captcha::spow::pow::Pow::init_random().unwrap();

        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

        let db = Database::connect(&database_url).await;
        let repos = match &db {
            Ok(db) => {
                shared::repositories::Repositories{

                    user_repo: UserRepository { pool: db.get_pool().clone(),},
                }
                
            },
            Err(e) => {
                println!("{:?}", e);
                panic!("Couldn't connect to database")
            }
            
        };
        let conf = get_configuration(None).await.unwrap();
        let mut leptos_options = conf.leptos_options;
        leptos_options.site_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 3000));
        let addr = leptos_options.site_addr;


        let routes = generate_route_list(App);
        let db = db.unwrap();
        let session_config = SessionConfig::default().with_table_name("axum_sessions").with_lifetime(chrono::Duration::days(7));
        let auth_config = AuthConfig::<i64>::default().set_cache(false);
        let session_store = SessionStore::<SessionMySqlPool>::new(
            Some(SessionMySqlPool::from(db.get_pool().clone())),
            session_config,
        )
        .await
        .unwrap();
        let state = AppState {
            options: leptos_options,
            repos: repos.clone(),
            routes: routes.clone(),
            db: db.clone(),
        };
        let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(fileserv::file_and_error_handler)
        .layer(AuthSessionLayer::<User, i64, SessionMySqlPool, MySqlPool>::new(
                Some(db.get_pool().clone()),
            )
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(state);
        log::info!("listening on http://{}", &addr);
        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener,app.into_make_service())
            .await
            .unwrap();
    }
}}
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}