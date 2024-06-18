use std::time::Duration;
use icondata::{BiPlusCircleRegular,BiEditAltRegular};

use ev::SubmitEvent;
use futures::TryFutureExt;
use itertools::Itertools;
use leptos::*;
use leptos_router::*;
use num_traits::{FromPrimitive, ToPrimitive};
use thaw::*;

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
    let action = create_server_action::<AddSensor>();
    view!{
        <ActionForm action=action on:submit=on_submit>
            <div class="mb-3">
                <label for="NameInput" class="form-label">Name</label>
                <input type="text" class="form-control" id="NameInput" aria-describedby="NameHelp" name="name"/>
                <div id="NameHelp" class="form-text">This is the Name that the Graph will have</div>
            </div>
            <div class="mb-3">
                <select class="form-select" aria-label="Select an graph" name="sensor_type">
                    <option selected value="">Select an Sensor Type</option>
                    <For
                    each=move || ["Dht22","Temp"].iter()
                    key=move |data| data.to_string()
                    let:data
                    > 
                        <option value=data.to_string()> {data.to_string()} </option>
                    </For>
                </select>
            </div>
            <div class="mb-3">
                <label for="PinInput" class="form-label">Pin</label>
                <input type="text" class="form-control" id="PinInput" aria-describedby="PinHelp" name="pin"/>
                <div id="PinHelp" class="form-text">"If you use an GPIO Sensor you don't need to fill in the Address field"</div>
            </div>
            <div class="mb-3">
                <label for="AddrInput" class="form-label">Address</label>
                <input type="text" class="form-control" id="AddrInput" aria-describedby="AddrHelp" name="addr"/>
                <div id="AddrHelp" class="form-text">"This is your I²C Address, if you use an I²C Sensor"</div>
            </div>
            <button type="submit" class="btn btn-primary">Submit</button>
        </ActionForm>
    }
}


#[server]
pub async fn add_sensor(name: String, sensor_type: String, pin: String, addr: String) -> Result<(),ServerFnError::<String>> {
    use crate::grpc::{add_new_sensor_addr,add_new_sensor_pin};
    if !addr.is_empty() && !pin.is_empty() {
        return Err(ServerFnError::ServerError("Invalid pin and addr can't have data".to_string()));
    }
    if !addr.is_empty() {
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