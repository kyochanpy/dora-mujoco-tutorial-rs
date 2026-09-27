use arrow::array::Float64Array;
use dora_node_api::{DoraNode, Event};
use eyre::Result;
use std::sync::Arc;

fn main() -> Result<()> {
    let (mut node, mut events) = DoraNode::init_from_env()?;

    const PERIOD: f64 = 4.0;
    const LIMIT: f64 = 2.0;

    while let Some(event) = events.recv() {
        if let Event::Input { data, .. } = event {
            let array: &Float64Array = data
                .as_any()
                .downcast_ref()
                .ok_or_else(|| eyre::eyre!("Failed to downcast input to Float64Array"))?;

            if array.is_empty() {
                continue;
            }

            let current_sim_time = array.value(0);
            let cycle = current_sim_time % PERIOD;
            let linear_value = (cycle - (PERIOD / 2.0)).abs() / (PERIOD / 4.0) - 1.0;
            let target_position = linear_value * LIMIT;
            let output_array = Float64Array::from(vec![target_position]);

            node.send_output(
                "action".to_string().into(),
                Default::default(),
                Arc::new(output_array) as Arc<dyn arrow::array::Array>, // Arc でラップ
            )?;
        }
    }
    Ok(())
}
