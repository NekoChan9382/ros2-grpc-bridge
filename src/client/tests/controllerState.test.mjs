import assert from "node:assert/strict";
import test from "node:test";
import { controllerToJoy, sameControllerState, snapshotController } from "../src/controllerState.ts";

function gamepad() {
  return {
    id: "Mobile gamepad", index: 0, mapping: "standard", connected: true, timestamp: 1,
    axes: [0, 0, 0, 0],
    buttons: Array.from({ length: 17 }, () => ({ pressed: false, touched: false, value: 0 })),
  };
}

test("snapshots do not retain mutable browser arrays or buttons", () => {
  const source = gamepad();
  const snapshot = snapshotController(source);
  source.axes[0] = 0.75;
  source.buttons[0].pressed = true;
  assert.equal(snapshot.axes[0], 0);
  assert.equal(snapshot.buttons[0].pressed, false);
  assert.equal(sameControllerState(snapshot, snapshotController(source)), false);
});

test("new browser objects with identical input do not trigger an input update", () => {
  const a = snapshotController(gamepad());
  const b = snapshotController({ ...gamepad(), timestamp: 2 });
  assert.equal(sameControllerState(a, b), true);
});

test("Joy uses the latest sampled axes, buttons, and analog triggers", () => {
  const old = snapshotController(gamepad());
  const next = gamepad();
  next.axes = [0.5, -0.25, 0.75, -1];
  next.buttons[0].pressed = true;
  next.buttons[12].pressed = true;
  next.buttons[6].value = 0.4;
  next.buttons[7].value = 0.9;
  const joy = controllerToJoy(snapshotController(next));
  assert.deepEqual(joy.axes, [0.5, -0.25, 0.75, -1, 0.4, 0.9]);
  assert.deepEqual(joy.buttons, [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  assert.deepEqual(controllerToJoy(old).axes, [0, 0, 0, 0, 0, 0]);
});

test("disconnects, device changes, and touch-only changes are detected", () => {
  const a = snapshotController(gamepad());
  assert.equal(sameControllerState(a, null), false);
  assert.equal(sameControllerState(null, null), true);
  assert.equal(sameControllerState(a, { ...a, index: 1 }), false);
  const touched = gamepad();
  touched.buttons[0].touched = true;
  assert.equal(sameControllerState(a, snapshotController(touched)), false);
});

test("missing axes and buttons default to zero", () => {
  const joy = controllerToJoy(snapshotController({ ...gamepad(), axes: [], buttons: [] }));
  assert.deepEqual(joy.axes, Array(6).fill(0));
  assert.deepEqual(joy.buttons, Array(14).fill(0));
});
