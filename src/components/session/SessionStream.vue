<script setup lang="ts">
import { computed, watch, ref } from "vue";
import { useSessionStore } from "../../stores/session";

const session = useSessionStore();
const streamRef = ref<HTMLElement | null>(null);

const currentTitle = computed(() => {
  const s = session.currentSession as { title?: string; id?: string } | null;
  return s?.title || s?.id || "No session selected";
});

function messageText(m: unknown): string {
  const item = m as { info?: { role?: string }; parts?: { type?: string; text?: string }[] };
  const parts = item.parts ?? [];
  const textParts = parts.filter((p) => p.type === "text" || !p.type).map((p) => p.text || "");
  return textParts.join("\n").trim() || "…";
}

function messageRole(m: unknown): string {
  const item = m as { info?: { role?: string } };
  return item.info?.role || "unknown";
}

watch(
  () => session.messages,
  () => {
    setTimeout(() => {
      if (streamRef.value) {
        streamRef.value.scrollTop = streamRef.value.scrollHeight;
      }
    }, 10);
  },
  { deep: true }
);
</script>

<template>
  <div class="stream">
    <div class="header">
      <div class="title">{{ currentTitle }}</div>
    </div>
    <div ref="streamRef" class="messages">
      <div v-if="!session.currentSession" class="empty">Select or create a session to begin.</div>
      <div
        v-for="(m, i) in session.messages"
        :key="i"
        class="message"
        :class="messageRole(m)"
      >
        <div class="role">{{ messageRole(m) }}</div>
        <pre class="text">{{ messageText(m) }}</pre>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stream {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.header {
  padding: var(--bc-space-md) var(--bc-space-lg);
  border-bottom: 1px solid var(--bc-border);
  background: rgba(11, 16, 22, 0.6);
}
.title {
  font-weight: 600;
  font-size: 14px;
  color: var(--bc-text);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--bc-space-md);
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-md);
}
.empty {
  color: var(--bc-text-dim);
  text-align: center;
  margin-top: var(--bc-space-xl);
}
.message {
  border-left: 2px solid var(--bc-border);
  padding-left: var(--bc-space-md);
}
.message.user {
  border-left-color: var(--bc-core);
}
.message.assistant {
  border-left-color: var(--bc-flame);
}
.role {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-xs);
}
.text {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: var(--bc-font-sans);
  font-size: 13px;
  color: var(--bc-text-muted);
}
</style>
