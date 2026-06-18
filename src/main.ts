import { invoke } from "@tauri-apps/api/core";

document.addEventListener("DOMContentLoaded", () => {
  const captureBtn = document.getElementById("captureBtn") as HTMLButtonElement;
  const statusDiv = document.getElementById("status") as HTMLDivElement;

  if (captureBtn) {
    captureBtn.addEventListener("click", async () => {
      try {
        const text = await navigator.clipboard.readText();
        statusDiv.innerHTML = `📋 Capturing ${text.length} characters...`;

        const result = await invoke<string>("capture_chat", { clipboardText: text });
        statusDiv.innerHTML = `✅ ${result}`;
      } catch (err) {
        console.error(err);
        statusDiv.innerHTML = `❌ Error: ${err}`;
      }
    });
  }
});