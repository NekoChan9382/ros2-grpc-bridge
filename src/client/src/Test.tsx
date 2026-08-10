import React from "react";
import { useGrpc } from "./GrpcContext";

export const Testdayo: React.FC = () => {
  const { client } = useGrpc();
  const joy = async () => {
    if (!client) return null;
    await client.setJoy({ leftX: 10.1, leftY: 12.2, rightX: 14.4, rightY: 16.6 })
  };
  return (<button onClick={joy}>Send</button>)
};
