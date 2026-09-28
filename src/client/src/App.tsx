import { GrpcProvider } from "./GrpcContext";
import { Testdayo } from "./Test";
import { ControllerProvider } from "./ControllerContext";

function App() {

  return (
    <GrpcProvider url="https://10.133.1.240:50051"><ControllerProvider><Testdayo /></ControllerProvider></GrpcProvider>
  )
}

export default App
