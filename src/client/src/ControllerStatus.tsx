import { useController } from "./ControllerContext";
import "./ControllerStatus.css";

const buttonLabels = [
  "A / ×", "B / ○", "X / □", "Y / △", "LB / L1", "RB / R1",
  "LT / L2", "RT / R2", "Back / Share", "Start / Options",
  "左スティック", "右スティック", "↑", "↓", "←", "→", "Home",
];

function StickPosition({ label, x, y }: { label: string; x: number; y: number }) {
  return (
    <div className="controller-status__stick">
      <h3>{label}</h3>
      <svg viewBox="0 0 100 100" role="img" aria-label={`${label}: X ${x.toFixed(3)}, Y ${y.toFixed(3)}`}>
        <circle cx="50" cy="50" r="44" className="controller-status__stick-outline" />
        <path d="M 6 50 H 94 M 50 6 V 94" className="controller-status__stick-grid" />
        <circle cx={50 + Math.max(-1, Math.min(1, x)) * 38} cy={50 + Math.max(-1, Math.min(1, y)) * 38} r="6" className="controller-status__stick-dot" />
      </svg>
      <span>X: {x.toFixed(3)} / Y: {y.toFixed(3)}</span>
    </div>
  );
}

export default function ControllerStatus() {
  const { controller, connected } = useController();
  const current = connected ? controller : null;

  return (
    <section className="controller-status" aria-labelledby="controller-status-title">
      <header className="controller-status__header">
        <h2 id="controller-status-title">コントローラーの状態</h2>
        <span className={`controller-status__connection ${current ? "is-connected" : ""}`} role="status">
          {current ? "接続中" : "未接続"}
        </span>
      </header>
      {!current ? <p>コントローラーを接続して、ボタンを押してください。</p> : (
        <>
          <p className="controller-status__id">{current.id}</p>
          <div className="controller-status__sticks">
            {current.axes.length >= 2 && <StickPosition label="左スティック" x={current.axes[0]} y={current.axes[1]} />}
            {current.axes.length >= 4 && <StickPosition label="右スティック" x={current.axes[2]} y={current.axes[3]} />}
          </div>
          <h3>軸</h3>
          <div className="controller-status__axes">
            {current.axes.map((value, index) => (
              <label key={index}>Axis {index}
                <meter min={-1} max={1} value={value} />
                <output>{value.toFixed(3)}</output>
              </label>
            ))}
          </div>
          <h3>ボタン</h3>
          <div className="controller-status__buttons">
            {current.buttons.map((button, index) => (
              <div key={index} className={`controller-status__button ${button.pressed ? "is-pressed" : ""}`}>
                <span>{buttonLabels[index] ?? `Button ${index}`}</span>
                <small>#{index} · {button.pressed ? "押下中" : button.touched ? "タッチ中" : "未押下"}</small>
                <meter min={0} max={1} value={button.value} aria-label={`Button ${index} 入力値`} />
                <output>{button.value.toFixed(3)}</output>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}
