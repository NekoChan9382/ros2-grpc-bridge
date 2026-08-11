import React, { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";

export const Testdayo: React.FC = () => {
  const { client } = useGrpc();

  const [axes, setAxes] = useState<number[]>([]);

  useEffect(() => {
    if (!client) return;
    let cancel = false;
    const receive = async () => {
      try {
        const stream = client.subscribeJoy({});
        for await (const joy of stream) {
          if (cancel) break;
          console.log("data came!");
          setAxes([joy.leftX, joy.leftY, joy.rightX, joy.rightY]);
        }
      } catch (e) { if (!cancel) { console.error("SubscribeJoy failed: ", e); } }
    }
    receive();
    console.log("subscription");
    return () => { cancel = true; }
  }, [client]);

  const joy = async () => {
    if (!client) return null;
    await client.setJoy({ leftX: 10.1, leftY: 12.2, rightX: 14.4, rightY: 16.6 })
  };
  return (<><button onClick={joy}>Send</button><p>axes: {axes.join(", ")}</p></>)
};
