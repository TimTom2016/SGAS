use leptos::{component, IntoView, SignalGet, SignalSet, view};
use leptos_struct_table::PaginationController;

#[component]
pub fn PageLink(page: usize, pagination_controller: PaginationController) -> impl IntoView {
	let is_selected = move || pagination_controller.current_page.get() == page;

	let class = move || {
		if is_selected() {
			"page-link active"
		} else {
			"page-link"
		}
	};

	view! {
        <li class="page-item">
            <a
                href="#"
                class=class
                on:click=move |evt| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    pagination_controller.current_page.set(page);
                }
            >

                {page + 1}
            </a>
        </li>
    }
}