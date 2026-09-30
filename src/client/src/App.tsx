import { GrpcProvider } from "./GrpcContext";
import { Testdayo } from "./Test";
import { ControllerProvider } from "./ControllerContext";

function App() {
  const grpcUrl = import.meta.env.VITE_GRPC_URL;
  if (!grpcUrl) {
    throw new Error("VITE_GRPC_URLが設定されていません");
  }

  return (
    <GrpcProvider url={grpcUrl}><ControllerProvider><Testdayo /></ControllerProvider></GrpcProvider>
  )
}

export default App
