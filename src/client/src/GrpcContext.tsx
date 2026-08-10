import React, { useContext, createContext, useMemo } from "react";
import { createClient } from "@connectrpc/connect"
import { createGrpcWebTransport } from "@connectrpc/connect-web";

import { RobotService } from "./gen/proto/robot_connect";

interface GrpcContextType { client: ReturnType<typeof createClient<typeof RobotService>> | null }

const GrpcContext = createContext<GrpcContextType>({ client: null });

export const useGrpc = () => { return useContext(GrpcContext) };

export const GrpcProvider: React.FC<{ url: string; children: React.ReactNode; }> = ({ url, children }) => {
  const transport = useMemo(() => createGrpcWebTransport({ baseUrl: url, }), [url]);
  const client = useMemo(() => createClient(RobotService, transport), [transport]);
  return (<GrpcContext value={{ client }}>{children}</GrpcContext>)
};
