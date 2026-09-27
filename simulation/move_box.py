import os

import mujoco
import mujoco.viewer
import pyarrow as pa
from dora import Node

# パス解決: simulation/ フォルダから一つ上がって assets/ を参照
MODEL_PATH = os.path.join(os.path.dirname(__file__), "../assets/move_box.xml")
model = mujoco.MjModel.from_xml_path(MODEL_PATH)
data = mujoco.MjData(model)
node = Node()

with mujoco.viewer.launch_passive(model, data) as viewer:
    while viewer.is_running():
        node.send_output("observation", pa.array([data.time], type=pa.float64()))
        event = node.next(timeout=0.001)

        if event is not None and event["type"] == "INPUT":
            if event["id"] == "control":
                value = event["value"].to_pylist()[0]
                data.ctrl[0] = value

        for _ in range(1):
            mujoco.mj_step(model, data)
        viewer.sync()
