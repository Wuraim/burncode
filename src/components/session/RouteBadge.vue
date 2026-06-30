<script setup lang="ts">
import { useSessionStore } from "../../stores/session";

const session = useSessionStore();
</script>

<template>
  <div v-if="session.route" class="route-badge">
    <span class="label">Route</span>
    <span class="value">{{ session.route.provider_id }} / {{ session.route.model_id }}</span>
    <span class="difficulty">{{ session.route.difficulty }}</span>
  </div>
</template>

<style scoped>
.route-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--bc-space-sm);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  border: 1px solid var(--bc-violet);
  background: rgba(123, 97, 255, 0.08);
  font-size: 11px;
  margin-bottom: var(--bc-space-sm);
}
.label {
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.value {
  color: var(--bc-core);
  font-family: var(--bc-font-mono);
}
.difficulty {
  color: var(--bc-flame);
  text-transform: uppercase;
  font-size: 9px;
}
</style>
