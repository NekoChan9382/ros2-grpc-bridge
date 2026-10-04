import React, { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";
import { useController } from "./ControllerContext";
import { controllerToJoy } from "./controllerState";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export const Testdayo: React.FC = () => {
  const { client } = useGrpc();
  const { connected, getController } = useController();
  const [axes, setAxes] = useState("");

  useEffect(() => {
    if (!client) return;
    const abort = new AbortController();
    const receive = async () => {
      try {
        const stream = client.subscribe(
          { topic: "/test", type: "std_msgs/msg/String" },
          { signal: abort.signal },
        );
        for await (const message of stream) {
          if (abort.signal.aborted) break;
          const data = JSON.parse(decoder.decode(message.json)) as { data: string };
          setAxes(data.data ?? "");
        }
      } catch (error) {
        if (!abort.signal.aborted) console.error("Subscribe failed:", error);
      }
    };
    void receive();
    return () => abort.abort();
  }, [client]);

  useEffect(() => {
    if (!client) return;
    const abort = new AbortController();
    let inFlight = false;
    const publish = setInterval(async () => {
      // Read the latest Context snapshot, not the value captured when the timer started.
      const controller = getController();
      if (!controller || inFlight) return;
      inFlight = true;
      try {
        const response = await client.publish({
          topic: "/joy",
          type: "sensor_msgs/msg/Joy",
          json: encoder.encode(JSON.stringify(controllerToJoy(controller))),
        }, { signal: abort.signal });
        if (!response.success) console.error("Publish rejected:", response.msg);
      } catch (error) {
        if (!abort.signal.aborted) console.error("Publish failed:", error);
      } finally {
        inFlight = false;
      }
    }, 10);
    return () => {
      clearInterval(publish);
      abort.abort();
    };
  }, [client, getController]);

  return (<><button>Send</button><p>axes: {axes}</p><p>Controller: {connected ? "Connected" : "Disconnected"}</p></>);
};
