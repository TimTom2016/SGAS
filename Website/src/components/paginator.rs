use leptos::{component, For, IntoView, SignalGet, view};
use leptos_struct_table::PaginationController;
use crate::components::page_link::PageLink;

#[component]
pub fn Paginator(pagination_controller: PaginationController) -> impl IntoView {
	let current_page = pagination_controller.current_page;
	let page_count = pagination_controller.page_count();

	let page_range = move || {
		let mut start = current_page().saturating_sub(2);

		let mut end = start + 5;

		if let Some(row_count) = page_count() {
			if end > row_count {
				end = row_count;
				start = end.saturating_sub(5);
			}
		}

		start..end
	};

	view! {
        <nav aria-label="Page navigation example" class="m-10 flex justify-end">
            <ul class="pagination">
                <li class="page-item" class:disabled=move || current_page.get() == page_range().start>
                    <a
                        href="#"
                        class="page-link"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.previous();
                        }
                    >

                        Previous
                    </a>
                </li>

                <For each=page_range key=|page| *page let:page>
                    <PageLink page pagination_controller />
                </For>

                <li class="page-item" class:disabled=move || current_page.get() == page_range().end-1>
                    <a
                        href="#"
                        class="page-link"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.next();
                        }
                    >

                        Next
                    </a>
                </li>
            </ul>
        </nav>
    }
}