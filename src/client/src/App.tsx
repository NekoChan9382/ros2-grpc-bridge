import './App.css'
import { GrpcProvider } from "./GrpcContext";
import { Testdayo } from "./Test";

function App() {

  return (
    <GrpcProvider url="http://localhost:50051"><Testdayo /></GrpcProvider>
  )
}

export default App
