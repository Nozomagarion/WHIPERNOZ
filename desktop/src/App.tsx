import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import StatusIndicator from "./components/StatusIndicator";

type AppState = "idle" | "recording" | "processing" | "done";

function App() {
  const [state, setState] = useState<AppState>("idle");
  const [transcript, setTranscript] = useState("");

  const toggleRecording = async () => {
    if (state === "idle") {
      setState("recording");
      try {
        const result = await invoke<string>("start_recording");
        setState("processing");
        setTranscript(result);
        setState("done");
        setTimeout(() => setState("idle"), 2000);
      } catch (e) {
        console.error("Recording error:", e);
        setState("idle");
      }
    } else if (state === "recording") {
      setState("processing");
      try {
        const result = await invoke<string>("stop_recording");
        setTranscript(result);
        setState("done");
        setTimeout(() => setState("idle"), 2000);
      } catch (e) {
        console.error("Stop error:", e);
        setState("idle");
      }
    }
  };

  return (
    <div className="container">
      <h1>Whipernoz</h1>
      <p className="subtitle">Voice-to-text dictation</p>

      <StatusIndicator state={state} />

      <button
        className={`record-btn ${state}`}
        onClick={toggleRecording}
        disabled={state === "processing"}
      >
        {state === "idle" && "Start Recording"}
        {state === "recording" && "Stop Recording"}
        {state === "processing" && "Processing..."}
        {state === "done" && "Done!"}
      </button>

      {transcript && (
        <div className="transcript">
          <h3>Transcript:</h3>
          <p>{transcript}</p>
        </div>
      )}
    </div>
  );
}

export default App;
