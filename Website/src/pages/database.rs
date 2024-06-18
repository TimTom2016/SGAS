use leptos::*;
use crate::components::sensor_table::SensorTable;
use crate::components::header::Header;
#[component]
pub fn Database() -> impl IntoView {
    view!{
        <div class="d-flex flex-column flex-fill">
            <Header/>
            <div class="m-4">
                <SensorTable/>
            </div>
        </div>
    }
}