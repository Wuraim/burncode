<script setup lang="ts">
import { computed } from "vue";
import RadarChart from "./RadarChart.vue";
import { useTelemetryStore } from "../../stores/telemetry";

const telemetry = useTelemetryStore();

const stats = computed(() => {
  const total = telemetry.requestCount || 1;
  const reliability = Math.max(0, 100 - (telemetry.errorCount / total) * 100);
  const activity = Math.min(telemetry.requestCount * 10, 100);
  return {
    latency: 40 + Math.min(telemetry.requestCount * 3, 55),
    reliability,
    cost: 25 + Math.min(telemetry.requestCount * 2, 50),
    context: 30 + Math.min(telemetry.requestCount * 4, 60),
    confidence: 50 + activity / 2,
  };
});

const values = computed(() => [
  stats.value.latency,
  stats.value.reliability,
  stats.value.cost,
  stats.value.context,
  stats.value.confidence,
]);

const labels = ["Latency", "Reliability", "Cost", "Context", "Confidence"];
</script>

<template>
  <div class="telemetry-panel">
    <div class="title">Telemetry</div>
    <div class="stats">
      <div class="stat">
        <div class="value">{{ telemetry.requestCount }}</div>
        <div class="label">Requests</div>
      </div>
      <div class="stat">
        <div class="value">{{ telemetry.errorCount }}</div>
        <div class="label">Errors</div>
      </div>
    </div>
    <RadarChart :values="values" :labels="labels" />
  </div>
</template>

<style scoped>
.telemetry-panel {
  padding: var(--bc-space-md);
  border-bottom: 1px solid var(--bc-border);
}
.title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-sm);
}
.stats {
  display: flex;
  gap: var(--bc-space-lg);
  margin-bottom: var(--bc-space-md);
}
.stat .value {
  font-size: 20px;
  font-weight: 700;
  color: var(--bc-core);
  font-family: var(--bc-font-mono);
}
.stat .label {
  font-size: 10px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
</style>
