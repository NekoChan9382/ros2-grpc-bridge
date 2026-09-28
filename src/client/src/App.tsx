import { GrpcProvider } from "./GrpcContext";
import { Testdayo } from "./Test";

function App() {

  return (
    <GrpcProvider url="https://10.133.1.240:50051"><Testdayo /></GrpcProvider>
  )
}

export default App
