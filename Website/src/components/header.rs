use icondata::BiUserCircleRegular;
use leptos::*;
use leptos_use::use_preferred_dark;
use thaw::*;
use crate::auth;


#[component]
pub fn Header() -> impl IntoView {
	let action = create_server_action::<auth::Logout>();
	return view!{
		<nav class="navbar navbar-expand-md d-flex sticky-top bg-dark" data-bs-theme="dark">
			<div class="container-fluid">
				<div class="collapse navbar-collapse " id="navbarNav">
					<ul class="navbar-nav me-auto navbar-nav-scroll">
						<li class="nav-item">
							<a class="nav-link active" aria-current="page" href="/">Home</a>
						</li>
						<li class="nav-item">
							<a class="nav-link" href="database">Database</a>
						</li>
						// <li class="nav-item">
						// 	<a class="nav-link disabled">Disabled</a>
						// </li>
					</ul>
					<div class="d-flex">
						<div class="dropdown">
							<a class="dropdown-toggle caret-off text-light" id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false" data-bs-display="static">
								<Icon icon=BiUserCircleRegular height="2em" width="2em" class="my-auto"/>
							</a>
							<ul class="dropdown-menu dropdown-menu-lg-end" aria-labelledby="dropdownMenuButton1">
								<li><a class="dropdown-item" href="login">Login</a></li>
								<li><a class="dropdown-item" href="signup">Signup</a></li>
								<li><Logout  action=action class="dropdown-item"/></li>
							</ul>
						</div>
					</div>
				</div>
			</div>
		</nav>
	}
}

#[component]
pub fn Logout(
	action: Action<auth::Logout, Result<(), ServerFnError>>,
	#[prop(optional, into)]
    class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		// <ActionForm action=action class=class>
        //     <button type="submit" class="btn btn-link text-reset text-decoration-none">
        //         "Log Out"
        //     </button>
        // </ActionForm>
		<a on:click=move|_| action.dispatch(auth::Logout {}) class=class href="#"> "Log Out"
		</a>
    }
}

