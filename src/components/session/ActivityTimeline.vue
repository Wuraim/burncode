<script setup lang="ts">
import { computed } from "vue";
import { useTelemetryStore } from "../../stores/telemetry";

const telemetry = useTelemetryStore();

const recent = computed(() => telemetry.events.slice(0, 40));

function eventClass(type: string): string {
  if (type === "server_connected") return "core";
  if (type === "request_started" || type === "model_triggered") return "flame";
  if (type === "request_failed" || type === "sse_error") return "error";
  return "";
}
</script>

<template>
  <div class="activity">
    <div class="title">Burn log</div>
    <div class="log">
      <div
        v-for="(e, i) in recent"
        :key="i"
        class="entry"
        :class="eventClass(e.event_type)"
      >
        <span class="dot"></span>
        <span class="type">{{ e.event_type }}</span>
      </div>
      <div v-if="recent.length === 0" class="empty">No events yet</div>
    </div>
  </div>
</template>

<style scoped>
.activity {
  height: 180px;
  min-height: 180px;
  border-top: 1px solid var(--bc-border);
  background: var(--bc-panel);
  display: flex;
  flex-direction: column;
}
.title {
  padding: var(--bc-space-sm) var(--bc-space-md);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--bc-text-dim);
  border-bottom: 1px solid var(--bc-border);
}
.log {
  flex: 1;
  overflow-y: auto;
  padding: var(--bc-space-sm) var(--bc-space-md);
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.entry {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  font-size: 11px;
  color: var(--bc-text-muted);
  font-family: var(--bc-font-mono);
}
.dot {
  width: 5px;
  height: 5px;
  background: var(--bc-text-dim);
}
.entry.core .dot {
  background: var(--bc-core);
  box-shadow: 0 0 6px var(--bc-core);
}
.entry.flame .dot {
  background: var(--bc-flame);
  box-shadow: 0 0 6px var(--bc-flame);
}
.entry.error .dot {
  background: var(--bc-error);
}
.empty {
  color: var(--bc-text-dim);
  font-size: 12px;
}
</style>
