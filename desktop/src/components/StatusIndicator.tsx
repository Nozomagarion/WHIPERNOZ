type AppState = "idle" | "recording" | "processing" | "done";

interface StatusIndicatorProps {
  state: AppState;
}

function StatusIndicator({ state }: StatusIndicatorProps) {
  return (
    <div className={`status-indicator ${state}`}>
      <div className="status-dot" />
      <span className="status-text">
        {state === "idle" && "Ready"}
        {state === "recording" && "Recording..."}
        {state === "processing" && "Processing..."}
        {state === "done" && "Text injected!"}
      </span>
    </div>
  );
}

export default StatusIndicator;
