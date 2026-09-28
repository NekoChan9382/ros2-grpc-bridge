import React, { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";
import { useController } from "./ControllerContext";
export const Testdayo: React.FC = () => {
  const { client } = useGrpc();
  const { controller, connected } = useController();
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  type joyMessage = {
    axes: number[];
    buttons: number[];
  };
  const controller_to_joy = (controller: Gamepad): joyMessage => {
    const axes = [
      controller.axes[0] ?? 0,
      controller.axes[1] ?? 0,
      controller.axes[2] ?? 0,
      controller.axes[3] ?? 0,
      controller.buttons[6]?.value ?? 0,
      controller.buttons[7]?.value ?? 0,
    ];
    const buttons = [
      controller.buttons[0]?.pressed ? 1 : 0,
      controller.buttons[1]?.pressed ? 1 : 0,
      controller.buttons[2]?.pressed ? 1 : 0,
      controller.buttons[3]?.pressed ? 1 : 0,
      controller.buttons[12]?.pressed ? 1 : 0,
      controller.buttons[13]?.pressed ? 1 : 0,
      controller.buttons[14]?.pressed ? 1 : 0,
      controller.buttons[15]?.pressed ? 1 : 0,
      controller.buttons[4]?.pressed ? 1 : 0,
      controller.buttons[5]?.pressed ? 1 : 0,
      controller.buttons[10]?.pressed ? 1 : 0,
      controller.buttons[11]?.pressed ? 1 : 0,
      controller.buttons[8]?.pressed ? 1 : 0,
      controller.buttons[9]?.pressed ? 1 : 0,
    ];

    return { axes, buttons };
  };

  const [axes, setAxes] = useState<string>("");

  useEffect(() => {
    if (!client) return;
    let cancel = false;
    const receive = async () => {
      try {
        const stream = client.subscribe({ topic: "/test", type: "std_msgs/msg/String" });
        for await (const message of stream) {
          if (cancel) break;
          const joy = JSON.parse(decoder.decode(message.json)) as { data: string };
          setAxes(joy.data ?? "");
        }
      } catch (e) { if (!cancel) { console.error("Subscribe failed: ", e); } }
    }
    receive();
    console.log("subscription");

    const publish = setInterval(async () => {
      if (!client || !controller || !connected) return null;
      const msg = controller_to_joy(controller);
      await client.publish({
        topic: "/joy",
        type: "sensor_msgs/msg/Joy",
        json: encoder.encode(JSON.stringify(msg)),
      });
    }, 10);
    return () => { cancel = true; clearInterval(publish); }
  }, [client, controller, connected]);

  return (<><button>Send</button><p>axes: {axes}</p><p>Controller: {connected ? "Connected" : "Disconencted"}</p></>)
};
