<script setup lang="ts">
import TodoPanel from "../todos/TodoPanel.vue";
import ProviderPanel from "../providers/ProviderPanel.vue";
import TelemetryPanel from "../telemetry/TelemetryPanel.vue";
</script>

<template>
  <aside class="right-rail">
    <TelemetryPanel />
    <ProviderPanel />
    <TodoPanel />
  </aside>
</template>

<style scoped>
.right-rail {
  width: 260px;
  min-width: 260px;
  background: var(--bc-panel);
  border-left: 1px solid var(--bc-border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
