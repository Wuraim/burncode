<script setup lang="ts">
import { onMounted } from "vue";
import ConnectPage from "./pages/ConnectPage.vue";
import WorkspacePage from "./pages/WorkspacePage.vue";
import { useConnectionStore } from "./stores/connection";
import { useTelemetryStore } from "./stores/telemetry";
import { useStreamingStore } from "./stores/streaming";

const connection = useConnectionStore();
const telemetry = useTelemetryStore();
const streaming = useStreamingStore();

onMounted(async () => {
  telemetry.startListening();
  streaming.startListening();
  await connection.loadSettings();
});
</script>

<template>
  <WorkspacePage v-if="connection.connected" />
  <ConnectPage v-else />
</template>

<style>
@import "./styles/base.css";
</style>
