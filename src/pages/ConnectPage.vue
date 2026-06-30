<script setup lang="ts">
import { onMounted } from "vue";
import ConnectionForm from "../components/connection/ConnectionForm.vue";
import { useConnectionStore } from "../stores/connection";
import type { ConnectionProfile } from "../types/app";

const connection = useConnectionStore();

onMounted(() => {
  connection.loadSettings();
});

async function onSubmit(profile: ConnectionProfile) {
  await connection.connect(profile);
}
</script>

<template>
  <div class="connect-page">
    <div class="card">
      <div class="brand">
        <h1>BURNCODE</h1>
        <p>Local opencode command deck</p>
      </div>
      <ConnectionForm :model-value="connection.profile" @submit="onSubmit" />
      <div v-if="connection.connecting" class="status">Establishing uplink…</div>
      <div v-if="connection.error" class="error">{{ connection.error }}</div>
    </div>
  </div>
</template>

<style scoped>
.connect-page {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at 50% 30%, rgba(76, 201, 240, 0.06), transparent 60%),
    var(--bc-bg);
}
.card {
  width: 420px;
  border: 1px solid var(--bc-border);
  background: var(--bc-panel);
  padding: var(--bc-space-xl);
  box-shadow: 0 16px 60px rgba(0, 0, 0, 0.5);
}
.brand {
  margin-bottom: var(--bc-space-xl);
}
.brand h1 {
  margin: 0;
  font-size: 28px;
  letter-spacing: 0.18em;
  background: linear-gradient(90deg, var(--bc-core), var(--bc-violet), var(--bc-magenta));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}
.brand p {
  margin: var(--bc-space-xs) 0 0;
  color: var(--bc-text-muted);
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
}
.status {
  margin-top: var(--bc-space-md);
  color: var(--bc-core);
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  animation: pulse 1.2s infinite;
}
.error {
  margin-top: var(--bc-space-md);
  color: var(--bc-ember);
  font-size: 12px;
  border-left: 2px solid var(--bc-ember);
  padding-left: var(--bc-space-sm);
}
@keyframes pulse {
  0%,
  100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}
</style>
