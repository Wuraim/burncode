<script setup lang="ts">
import { ref } from "vue";
import { useSessionStore } from "../../stores/session";
import { useAgentsStore } from "../../stores/agents";
import { useStreamingStore } from "../../stores/streaming";
import RouteBadge from "./RouteBadge.vue";
import AgentSelector from "./AgentSelector.vue";

const session = useSessionStore();
const agents = useAgentsStore();
const streaming = useStreamingStore();
const text = ref("");

async function submit() {
  const t = text.value.trim();
  if (!t) return;
  text.value = "";
  await session.sendPromptAsync(t, agents.selectedName || undefined);
}

async function abort() {
  const ses = session.currentSession as { id?: string } | null;
  streaming.finishStreaming();
  if (ses?.id) {
    await session.abortSession(ses.id);
  }
}
</script>

<template>
  <div class="composer">
    <div class="row">
      <RouteBadge />
      <AgentSelector />
    </div>
    <textarea
      v-model="text"
      :disabled="session.sending || streaming.isStreaming"
      placeholder="Ignite a task…"
      rows="3"
      @keydown.enter.prevent.exact="submit"
    />
    <div class="bar">
      <div class="meta">{{ streaming.isStreaming ? "Burning…" : session.sending ? "Sending…" : "Ready" }}</div>
      <div class="actions">
        <button v-if="streaming.isStreaming" class="abort" @click="abort">Abort</button>
        <button class="primary" :disabled="session.sending || streaming.isStreaming || !text.trim()" @click="submit">
          Fire
        </button>
      </div>
    </div>
    <div v-if="session.error" class="error">{{ session.error }}</div>
  </div>
</template>

<style scoped>
.composer {
  border-top: 1px solid var(--bc-border);
  padding: var(--bc-space-md);
  background: var(--bc-panel);
}
.composer .row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--bc-space-md);
  margin-bottom: var(--bc-space-sm);
}
textarea {
  width: 100%;
  resize: none;
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  color: var(--bc-text);
  padding: var(--bc-space-md);
  font-family: var(--bc-font-sans);
  font-size: 13px;
  line-height: 1.5;
}
textarea:focus {
  border-color: var(--bc-core);
  outline: none;
}
.bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--bc-space-sm);
}
.meta {
  font-size: 11px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.actions {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
}
button {
  min-width: 80px;
}
.abort {
  background: transparent;
  border-color: var(--bc-error);
  color: var(--bc-error);
}
.error {
  margin-top: var(--bc-space-sm);
  color: var(--bc-ember);
  font-size: 12px;
}
</style>
