import { useContext, createContext, useState, useEffect } from "react";

interface ControllerContextType {
  controller: Gamepad | null;
  connected: boolean;
}

const ControllerContext = createContext<ControllerContextType>({
  controller: null,
  connected: false,
});

export const useController = () => {
  return useContext(ControllerContext);
};

export const ControllerProvider: React.FC<{
  children: React.ReactNode;
}> = ({ children }) => {
  const [controller, setController] = useState<Gamepad | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const handleGamepadConnected = (event: GamepadEvent) => {
      console.log("Gamepad connected:", event.gamepad);
      if (
        event.gamepad.id.toLowerCase().includes("controller") &&
        event.gamepad.mapping === "standard"
      ) {
        setController(event.gamepad);
        setConnected(true);
      }
    };

    const handleGamepadDisconnected = (event: GamepadEvent) => {
      if (
        event.gamepad.id.toLowerCase().includes("controller") &&
        event.gamepad.mapping === "standard"
      ) {
        setController(null);
        setConnected(false);
      }
    };

    window.addEventListener("gamepadconnected", handleGamepadConnected);
    window.addEventListener("gamepaddisconnected", handleGamepadDisconnected);

    return () => {
      window.removeEventListener("gamepadconnected", handleGamepadConnected);
      window.removeEventListener(
        "gamepaddisconnected",
        handleGamepadDisconnected,
      );
    };
  }, []);

  useEffect(() => {
    try {
      const gamepads = navigator.getGamepads();
      console.log("Detected gamepads:", gamepads);
      const dsController = gamepads.find(
        (gp) =>
          gp &&
          gp.id.toLowerCase().includes("controller") &&
          gp.mapping === "standard",
      );
      if (dsController) {
        setController(dsController);
        setConnected(true);
      }
    } catch (error) {
      console.error("Failed to initialize Controller controller:", error);
    }
  }, []);

  const value: ControllerContextType = { controller, connected };
  return <ControllerContext value={value}>{children}</ControllerContext>;
};
