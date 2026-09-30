import { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";

type Samples = { latest: number; mean: number; p95: number; max: number; count: number };
const summarize = (values: number[]): Samples | null => {
  if (!values.length) return null;
  const sorted = [...values].sort((a, b) => a - b);
  return {
    latest: values[values.length - 1],
    mean: values.reduce((a, b) => a + b, 0) / values.length,
    p95: sorted[Math.ceil(sorted.length * 0.95) - 1],
    max: sorted[sorted.length - 1], count: values.length,
  };
};

export function useJoyLatency(
  index: number | undefined,
  connected: boolean,
  toJoy: (gamepad: Gamepad) => { axes: number[]; buttons: number[] },
) {
  const { client } = useGrpc();
  const [stats, setStats] = useState({
    ack: null as Samples | null, echo: null as Samples | null,
    sent: 0, received: 0, expired: 0, failed: 0, skipped: 0, pending: 0,
    error: "", stream: "接続待ち",
  });

  useEffect(() => {
    if (!client || !connected || index === undefined) return;
    const abort = new AbortController();
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();
    const pending = new Map<string, number>();
    const ack: number[] = [];
    const echo: number[] = [];
    let sent = 0, received = 0, expired = 0, failed = 0, skipped = 0, inFlight = 0;
    let error = "", stream = "購読開始（最初の受信待ち）";
    let lastStamp = 0n;
    const record = (samples: number[], value: number) => {
      samples.push(value);
      if (samples.length > 1000) samples.shift();
    };
    const expire = (now: number) => {
      for (const [key, start] of pending) {
        if (now - start >= 5000) { pending.delete(key); expired++; }
      }
    };

    void (async () => {
      try {
        for await (const message of client.subscribe(
          { topic: "/joy", type: "sensor_msgs/msg/Joy" }, { signal: abort.signal },
        )) {
          // Include JSON decode and correlation in the measurement endpoint.
          const joy = JSON.parse(decoder.decode(message.json));
          const stamp = joy.header?.stamp;
          if (!stamp) continue;
          const key = `${stamp.sec}:${stamp.nanosec}`;
          const start = pending.get(key);
          if (start === undefined) continue;
          pending.delete(key);
          const elapsed = performance.now() - start;
          if (elapsed >= 5000) { expired++; continue; }
          received++;
          stream = "受信中";
          record(echo, elapsed);
        }
        if (!abort.signal.aborted) stream = "購読終了（コントローラー再接続で再試行）";
      } catch (cause) {
        if (!abort.signal.aborted) { error = String(cause); stream = "購読エラー"; }
      }
    })();

    const timer = window.setInterval(() => {
      if (document.hidden) { skipped++; return; }
      const now = performance.now();
      expire(now);
      // Preserve the requested 10ms cadence, but bound outstanding RPCs.
      if (inFlight >= 100) { skipped++; return; }
      const gamepad = navigator.getGamepads()[index];
      if (!gamepad?.connected) return;
      // Wall-clock acquisition stamp for ROS; elapsed time uses only performance.now().
      const wallNs = BigInt(Date.now()) * 1_000_000n;
      lastStamp = wallNs > lastStamp ? wallNs : lastStamp + 1n;
      const stamp = { sec: Number(lastStamp / 1_000_000_000n), nanosec: Number(lastStamp % 1_000_000_000n) };
      const key = `${stamp.sec}:${stamp.nanosec}`;
      pending.set(key, now);
      const json = encoder.encode(JSON.stringify({ ...toJoy(gamepad), header: { stamp, frame_id: "" } }));
      sent++;
      inFlight++;
      void client.publish({ topic: "/joy", type: "sensor_msgs/msg/Joy", json }, {
        signal: abort.signal, timeoutMs: 5000,
      }).then((response) => {
        if (abort.signal.aborted) return;
        if (!response.success) throw new Error(response.msg);
        record(ack, performance.now() - now);
      }).catch((cause) => {
        if (!abort.signal.aborted) { failed++; error = String(cause); }
      }).finally(() => { inFlight--; });
    }, 10);

    const display = window.setInterval(() => {
      expire(performance.now());
      setStats({ ack: summarize(ack), echo: summarize(echo), sent, received,
        expired, failed, skipped, pending: pending.size, error, stream });
    }, 250);
    return () => { abort.abort(); clearInterval(timer); clearInterval(display); };
  }, [client, index, connected, toJoy]);
  return stats;
}
