use std::path::Path;
use std::time::Duration;

use dora_node_api::arrow::array::{Array, Float64Array};
use dora_node_api::{DoraNode, Event};
use mujoco_rs::prelude::MjModel;
use mujoco_rs::viewer::MjViewer;
use mujoco_rs::{mj_step, MjData, MjModel};

fn main() -> eyre::Result<()> {
    let model_path = "assets/move_box.xml";

    let model = MjModel::from_xml(model_path).expect("Failed to load XML model.");
    let mut data = model.make_data();

    let mut viewer = MjViewer::launch_passive(&model, 0).expect("Failed to launch viewer.");

    let (mut node, mut events) = DoraNode::init_from_env()?;

    while viewer.running() {
        let time_array = Float64Array::from(vec![data.time]);
        node.send_output("observation".into(), Default::default(), time_array.into())?;

        if let Some(event) = events.recv_timeout(Duration::from_millis(1)) {
            match event {
                Event::Input {
                    id,
                    data: arrow_data,
                    ..
                } => {
                    if id.as_str() == "control" {
                        if let Some(float_array) =
                            arrow_data.as_any().downcast_ref::<Float64Array>()
                        {
                            if !float_array.is_empty() {
                                data.ctrl[0] = float_array.value(0);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        mj_step(&model, &mut data);
        viewer.sync_data(&mut data);
    }
    Ok(())
}
