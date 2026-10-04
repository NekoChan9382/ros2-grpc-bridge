export interface ControllerState {
  id: string;
  index: number;
  mapping: GamepadMappingType;
  connected: boolean;
  timestamp: number;
  axes: readonly number[];
  buttons: readonly { pressed: boolean; touched: boolean; value: number }[];
}

export function snapshotController(gamepad: Gamepad): ControllerState {
  return {
    id: gamepad.id, index: gamepad.index, mapping: gamepad.mapping,
    connected: gamepad.connected, timestamp: gamepad.timestamp,
    axes: [...gamepad.axes],
    buttons: gamepad.buttons.map(({ pressed, touched, value }) => ({ pressed, touched, value })),
  };
}

export function sameControllerState(a: ControllerState | null, b: ControllerState | null): boolean {
  if (!a || !b) return a === b;
  return a.id === b.id && a.index === b.index && a.mapping === b.mapping &&
    a.connected === b.connected && a.axes.length === b.axes.length &&
    a.axes.every((value, index) => value === b.axes[index]) &&
    a.buttons.length === b.buttons.length && a.buttons.every((button, index) => {
      const other = b.buttons[index];
      return button.pressed === other.pressed && button.touched === other.touched && button.value === other.value;
    });
}

export function controllerToJoy(controller: ControllerState) {
  return {
    axes: [
      controller.axes[0] ?? 0, controller.axes[1] ?? 0,
      controller.axes[2] ?? 0, controller.axes[3] ?? 0,
      controller.buttons[6]?.value ?? 0, controller.buttons[7]?.value ?? 0,
    ],
    buttons: [0, 1, 2, 3, 12, 13, 14, 15, 4, 5, 10, 11, 8, 9].map(
      (index) => controller.buttons[index]?.pressed ? 1 : 0,
    ),
  };
}
