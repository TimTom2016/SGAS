use std::time::Duration;
use icondata::{BiPlusCircleRegular,BiEditAltRegular};

use ev::SubmitEvent;
use futures::TryFutureExt;
use itertools::Itertools;
use leptos::*;
use leptos_router::*;
use num_traits::{FromPrimitive, ToPrimitive};
use thaw::*;

use crate::grpc::{get_supported_sensor_types};

#[component]
pub fn AddSensor() -> impl IntoView {
    let mut got_triggered = create_rw_signal(false);
    let on_submit = move |ev: SubmitEvent| {
        let data = AddSensor::from_event(&ev);
        // silly example of validation: if the todo is "nope!", nope it
        let is_err = data.is_err();
        let input = data.unwrap();
        if is_err || input.sensor_type == "" || (!input.addr.is_empty() && !input.pin.is_empty()) {
            // ev.prevent_default() will prevent form submission
            ev.prevent_default();
        }
        got_triggered.set(true);
    };
    let i2c = create_resource(
        || (), |_| {
            async  move {
                let sensor_types =  get_supported_sensor_types().await.unwrap_or(vec![]);
                sensor_types
            }
        });
    let action = create_server_action::<AddSensor>();
    view!{
        <Suspense>
            <ActionForm action=action on:submit=on_submit>
                <div class="mb-3">
                    <label for="NameInput" class="form-label">Name</label>
                    <input type="text" class="form-control" id="NameInput" aria-describedby="NameHelp" name="name"/>
                    <div id="NameHelp" class="form-text">This is the Name that the Graph will have</div>
                </div>
                
                <div class="mb-3">
                    <label for="PinInput" class="form-label">Pin</label>
                    <input type="text" class="form-control" id="PinInput" aria-describedby="PinHelp" name="pin"/>
                    <div id="PinHelp" class="form-text">"If you use an GPIO Sensor you don't need to select an Sensor Type"</div>
                </div>
                <div class="mb-3 d-flex align-items-center">
                    <select class="form-select" aria-label="Select an Sensor Type" name="sensor_type">
                        <option selected value="GPIO">Select an Sensor Type</option>
                        <For
                        each=move || i2c.get().unwrap_or(vec![])
                        key=move |data| data.to_string()
                        let:data
                        > 
                            <option value=data.to_string()> {data.to_string()} </option>
                        </For>
                    </select>
                </div>
                <input type="hidden" name="addr" value=""/>
                <button type="submit" class="btn btn-primary">Submit</button>
            </ActionForm>
        </Suspense>
    }
}


#[server]
pub async fn add_sensor(name: String, sensor_type: String, pin: String, addr: String) -> Result<(),ServerFnError::<String>> {
    use crate::grpc::{add_new_sensor_addr,add_new_sensor_pin};
    if !addr.is_empty() && !pin.is_empty() {
        return Err(ServerFnError::ServerError("Invalid pin and addr can't have data".to_string()));
    }
    if !(sensor_type=="GPIO") {
        return add_new_sensor_addr(name, sensor_type, addr).await;
    } else {
        return add_new_sensor_pin(name, sensor_type, pin.parse::<i32>().unwrap()).await;
    }
}

#[component]
pub fn AddSensorButton() -> impl IntoView {
    let show = create_rw_signal(false);
    view!{
        <div class="d-flex justify-content-center">
            <button class="btn btn-primary" on:click=move |_| show.set(true)>
                Add Sensor
            </button> 
        </div>
        <Modal show>
            <div class="d-flex justify-content-center align-items-center">
                <AddSensor/>
            </div>
        </Modal>
    }
}