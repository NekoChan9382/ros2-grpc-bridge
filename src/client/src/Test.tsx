import React, { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";

export const Testdayo: React.FC = () => {
  const { client } = useGrpc();
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  const [axes, setAxes] = useState<number[]>([]);

  useEffect(() => {
    if (!client) return;
    let cancel = false;
    const receive = async () => {
      try {
        const stream = client.subscribe({ topic: "/joy", type: "sensor_msgs/msg/Joy" });
        for await (const message of stream) {
          if (cancel) break;
          const joy = JSON.parse(decoder.decode(message.json)) as { axes?: number[] };
          setAxes(joy.axes ?? []);
        }
      } catch (e) { if (!cancel) { console.error("Subscribe failed: ", e); } }
    }
    receive();
    console.log("subscription");
    return () => { cancel = true; }
  }, [client]);

  const joy = async () => {
    if (!client) return null;
    await client.publish({
      topic: "/joy",
      type: "sensor_msgs/msg/Joy",
      json: encoder.encode(JSON.stringify({ axes: [10.1, 12.2, 14.4, 16.6], buttons: [] })),
    });
  };
  return (<><button onClick={joy}>Send</button><p>axes: {axes.join(", ")}</p></>)
};
