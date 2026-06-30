import { defineStore } from "pinia";
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import type { OpencodeEvent } from "../types/app";

export const useTelemetryStore = defineStore("telemetry", () => {
  const events = ref<OpencodeEvent[]>([]);
  const requestCount = ref(0);
  const errorCount = ref(0);

  function pushEvent(event: OpencodeEvent) {
    events.value.unshift(event);
    if (events.value.length > 200) {
      events.value.pop();
    }
    if (event.event_type === "request_started") requestCount.value++;
    if (event.event_type === "request_failed") errorCount.value++;
  }

  async function startListening() {
    await listen<OpencodeEvent>("opencode-event", (e) => {
      pushEvent(e.payload);
    });
  }

  return {
    events,
    requestCount,
    errorCount,
    pushEvent,
    startListening,
  };
});
