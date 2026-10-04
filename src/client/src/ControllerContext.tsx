import { useContext, createContext, useState, useEffect, useRef, useCallback } from "react";
import { sameControllerState, snapshotController } from "./controllerState";
import type { ControllerState } from "./controllerState";

interface ControllerContextType {
  controller: ControllerState | null;
  connected: boolean;
  getController: () => ControllerState | null;
}

const ControllerContext = createContext<ControllerContextType>({
  controller: null,
  connected: false,
  getController: () => null,
});

// Keep the consumer hook alongside its private Context.
// eslint-disable-next-line react-refresh/only-export-components
export const useController = () => {
  return useContext(ControllerContext);
};

export const ControllerProvider: React.FC<{
  children: React.ReactNode;
}> = ({ children }) => {
  const [controller, setController] = useState<ControllerState | null>(null);
  const latestController = useRef<ControllerState | null>(null);
  const getController = useCallback(() => latestController.current, []);

  useEffect(() => {
    if (typeof navigator.getGamepads !== "function") return;
    let frameId: number;
    let errorReported = false;
    const readController = () => {
      let next: ControllerState | null = null;
      try {
        const gamepads = navigator.getGamepads();
        const selected = latestController.current;
        const previous = selected ? gamepads[selected.index] : null;
        // Keep the selected device while connected; IDs vary between browsers.
        const gamepad = previous?.connected && previous.mapping === "standard"
          ? previous
          : gamepads.find((gp) => gp?.connected && gp.mapping === "standard");
        if (gamepad) next = snapshotController(gamepad);
      } catch (error) {
        if (!errorReported) console.error("Failed to read controller:", error);
        errorReported = true;
      }
      const changed = !sameControllerState(latestController.current, next);
      latestController.current = next;
      if (changed) setController(next);
    };
    const poll = () => {
      readController();
      frameId = requestAnimationFrame(poll);
    };
    window.addEventListener("gamepadconnected", readController);
    window.addEventListener("gamepaddisconnected", readController);
    frameId = requestAnimationFrame(poll);
    return () => {
      cancelAnimationFrame(frameId);
      window.removeEventListener("gamepadconnected", readController);
      window.removeEventListener("gamepaddisconnected", readController);
    };
  }, []);

  const value: ControllerContextType = { controller, connected: controller !== null, getController };
  return <ControllerContext value={value}>{children}</ControllerContext>;
};
