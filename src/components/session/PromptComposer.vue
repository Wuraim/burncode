<script setup lang="ts">
import { ref } from "vue";
import { useSessionStore } from "../../stores/session";
import RouteBadge from "./RouteBadge.vue";

const session = useSessionStore();
const text = ref("");

async function submit() {
  const t = text.value.trim();
  if (!t) return;
  text.value = "";
  await session.sendPrompt(t);
}
</script>

<template>
  <div class="composer">
    <RouteBadge />
    <textarea
      v-model="text"
      :disabled="session.sending"
      placeholder="Ignite a task…"
      rows="3"
      @keydown.enter.prevent.exact="submit"
    />
    <div class="bar">
      <div class="meta">{{ session.sending ? "Burning…" : "Ready" }}</div>
      <button class="primary" :disabled="session.sending || !text.trim()" @click="submit">
        Fire
      </button>
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
button {
  min-width: 80px;
}
.error {
  margin-top: var(--bc-space-sm);
  color: var(--bc-ember);
  font-size: 12px;
}
</style>
