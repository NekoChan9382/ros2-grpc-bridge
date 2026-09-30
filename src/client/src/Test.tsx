import React, { useEffect, useState } from "react";
import { useGrpc } from "./GrpcContext";
import { useController } from "./ControllerContext";
import { useJoyLatency } from "./useJoyLatency";

  type joyMessage = {
    axes: number[];
    buttons: number[];
  };
  const controller_to_joy = (controller: Gamepad): joyMessage => {
    const axes = [
      controller.axes[0] ?? 0,
      controller.axes[1] ?? 0,
      controller.axes[2] ?? 0,
      controller.axes[3] ?? 0,
      controller.buttons[6]?.value ?? 0,
      controller.buttons[7]?.value ?? 0,
    ];
    const buttons = [
      controller.buttons[0]?.pressed ? 1 : 0,
      controller.buttons[1]?.pressed ? 1 : 0,
      controller.buttons[2]?.pressed ? 1 : 0,
      controller.buttons[3]?.pressed ? 1 : 0,
      controller.buttons[12]?.pressed ? 1 : 0,
      controller.buttons[13]?.pressed ? 1 : 0,
      controller.buttons[14]?.pressed ? 1 : 0,
      controller.buttons[15]?.pressed ? 1 : 0,
      controller.buttons[4]?.pressed ? 1 : 0,
      controller.buttons[5]?.pressed ? 1 : 0,
      controller.buttons[10]?.pressed ? 1 : 0,
      controller.buttons[11]?.pressed ? 1 : 0,
      controller.buttons[8]?.pressed ? 1 : 0,
      controller.buttons[9]?.pressed ? 1 : 0,
    ];

    return { axes, buttons };
  };

export const Testdayo: React.FC = () => {
  const { client } = useGrpc();
  const { controller, connected } = useController();
  const latency = useJoyLatency(controller?.index, connected, controller_to_joy);
  const [axes, setAxes] = useState<string>("");

  useEffect(() => {
    if (!client) return;
    const abort = new AbortController();
    const decoder = new TextDecoder();
    const receive = async () => {
      try {
        const stream = client.subscribe({ topic: "/test", type: "std_msgs/msg/String" }, { signal: abort.signal });
        for await (const message of stream) {
          if (abort.signal.aborted) break;
          const joy = JSON.parse(decoder.decode(message.json)) as { data: string };
          setAxes(joy.data ?? "");
        }
      } catch (e) { if (!abort.signal.aborted) { console.error("Subscribe failed: ", e); } }
    }
    receive();
    return () => { abort.abort(); };
  }, [client]);

  return (<>
    <p>axes: {axes}</p>
    <p>Controller: {connected ? "Connected" : "Disconnected（測定停止・最終値を表示）"}</p>
    <p>/joy: {latency.stream}</p>
    <table>
      <caption>遅延（ms）・各経路の直近最大1000件</caption>
      <thead><tr><th>測定区間</th><th>件数</th><th>最新</th><th>平均</th><th>p95</th><th>最大</th></tr></thead>
      <tbody>{([
        ["Publish 応答まで", latency.ack], ["ROS subscription 経由の往復", latency.echo],
      ] as const).map(([label, sample]) => <tr key={label}>
        <th>{label}</th><td>{sample?.count ?? 0}</td>
        {(["latest", "mean", "p95", "max"] as const).map(key => <td key={key}>{sample?.[key].toFixed(2) ?? "—"}</td>)}
      </tr>)}</tbody>
    </table>
    <p>送信: {latency.sent} / 往復受信: {latency.received} / 待機: {latency.pending} /
      5秒以内に未受信: {latency.expired} / Publish失敗: {latency.failed} / 送信スキップ: {latency.skipped}</p>
    <p>10ms間隔で送信。非表示タブでは停止。再接続時に統計をリセットします。
      初回接続・ROS discovery を含みます。片道遅延や物理入力からの遅延ではありません。</p>
    {latency.error && <p role="alert">{latency.error}</p>}
  </>);
};
