<script setup lang="ts">
import { computed, watch, ref } from "vue";
import { useSessionStore } from "../../stores/session";
import { useStreamingStore, type StreamingPart } from "../../stores/streaming";

const session = useSessionStore();
const streaming = useStreamingStore();
const streamRef = ref<HTMLElement | null>(null);
const showReasoning = ref<Set<string>>(new Set());

const currentTitle = computed(() => {
  const s = session.currentSession as { title?: string; id?: string } | null;
  return s?.title || s?.id || "No session selected";
});

function currentSessionId(): string {
  const s = session.currentSession as { id?: string } | null;
  return s?.id || "";
}

function messageId(m: unknown): string {
  const item = m as { info?: { id?: string } };
  return item.info?.id || "";
}

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

function toggleReasoning(id: string) {
  if (showReasoning.value.has(id)) {
    showReasoning.value.delete(id);
  } else {
    showReasoning.value.add(id);
  }
}

function partKey(p: StreamingPart, idx: number): string {
  return "id" in p ? (p as { id: string }).id || `idx-${idx}` : `idx-${idx}`;
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

watch(
  () => streaming.orderedParts.length,
  () => {
    setTimeout(() => {
      if (streamRef.value) {
        streamRef.value.scrollTop = streamRef.value.scrollHeight;
      }
    }, 10);
  }
);
</script>

<template>
  <div class="stream">
    <div class="header">
      <div class="title">{{ currentTitle }}</div>
    </div>
    <div ref="streamRef" class="messages">
      <div v-if="!session.currentSession && !streaming.isStreaming" class="empty">
        Select or create a session to begin.
      </div>

      <!-- Historical messages -->
      <div
        v-for="(m, i) in session.messages"
        :key="'hist-' + i"
        class="message"
        :class="messageRole(m)"
      >
        <div class="role">{{ messageRole(m) }}</div>
        <pre class="text">{{ messageText(m) }}</pre>
        <div class="message-actions">
          <button
            class="fork-btn"
            title="Fork session from here"
            @click="session.forkSession(currentSessionId(), messageId(m))"
          >
            Fork
          </button>
        </div>
      </div>

      <!-- Streaming content -->
      <div v-if="streaming.isStreaming" class="message assistant streaming">
        <div class="role">assistant</div>
        <div class="stream-body">
          <!-- Permission requests -->
          <div v-for="perm in streaming.permissionRequests" :key="perm.id" class="permission-block">
            <div class="perm-header">{{ perm.title }}</div>
            <div class="perm-type">{{ perm.type }}</div>
            <div class="perm-actions">
              <button class="perm-allow" @click="streaming.respondToPermission(perm.id, 'allow')">Allow</button>
              <button class="perm-deny" @click="streaming.respondToPermission(perm.id, 'deny')">Deny</button>
              <label class="perm-remember">
                <input type="checkbox" />
                <span>Remember</span>
              </label>
            </div>
          </div>

          <template v-for="(part, idx) in streaming.orderedParts" :key="partKey(part, idx)">
            <!-- Step start -->
            <div v-if="part.type === 'step-start'" class="step-boundary">
              <span class="step-line"></span>
            </div>

            <!-- Step finish -->
            <div v-else-if="part.type === 'step-finish'" class="step-boundary step-end">
              <span class="step-line"></span>
              <span class="step-reason">{{ part.reason }}</span>
            </div>

            <!-- Reasoning -->
            <div v-else-if="part.type === 'reasoning'" class="reasoning-block">
              <button class="reasoning-toggle" @click="toggleReasoning(part.id)">
                {{ showReasoning.has(part.id) ? "▾" : "▸" }} reasoning
              </button>
              <pre v-if="showReasoning.has(part.id)" class="reasoning-text">{{ part.text }}</pre>
            </div>

            <!-- Tool call -->
            <div v-else-if="part.type === 'tool'" class="tool-block" :class="'tool-' + part.status">
              <div class="tool-header">
                <span class="tool-name">{{ part.tool }}</span>
                <span class="tool-status">{{ part.status }}</span>
              </div>
              <div v-if="part.status === 'running'" class="tool-pulse"></div>
              <div v-if="part.output" class="tool-output">
                <pre>{{ part.output }}</pre>
              </div>
              <div v-if="part.error" class="tool-error">{{ part.error }}</div>
            </div>

            <!-- Text -->
            <pre v-else-if="part.type === 'text'" class="text streaming-text">
              {{ part.text || "" }}<span v-if="streaming.isStreaming && idx === streaming.orderedParts.length - 1" class="cursor">▍</span>
            </pre>
          </template>
        </div>
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
.message-actions {
  display: none;
  margin-top: var(--bc-space-xs);
}
.message:hover .message-actions {
  display: flex;
  gap: var(--bc-space-xs);
}
.fork-btn {
  font-size: 10px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
  background: transparent;
  border: 1px solid var(--bc-border);
  color: var(--bc-text-dim);
}
.fork-btn:hover {
  border-color: var(--bc-core);
  color: var(--bc-core);
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
  line-height: 1.5;
}
.stream-body {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
.streaming-text {
  color: var(--bc-text);
}
.cursor {
  animation: blink 0.8s infinite;
  color: var(--bc-core);
}
@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}
.step-boundary {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  margin: var(--bc-space-xs) 0;
}
.step-line {
  flex: 1;
  height: 1px;
  background: var(--bc-border);
}
.step-end .step-line {
  background: var(--bc-violet);
}
.step-reason {
  font-size: 10px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.reasoning-block {
  background: rgba(123, 97, 255, 0.05);
  border-left: 2px solid var(--bc-violet);
  padding: var(--bc-space-sm) var(--bc-space-md);
}
.reasoning-toggle {
  font-size: 11px;
  color: var(--bc-violet);
  background: none;
  border: none;
  cursor: pointer;
  font-family: var(--bc-font-mono);
}
.reasoning-text {
  font-size: 12px;
  color: var(--bc-text-dim);
  margin: var(--bc-space-sm) 0 0;
  white-space: pre-wrap;
  font-family: var(--bc-font-mono);
  line-height: 1.4;
}
.tool-block {
  border: 1px solid var(--bc-border);
  border-left: 2px solid var(--bc-core);
  padding: var(--bc-space-sm) var(--bc-space-md);
  font-size: 12px;
}
.tool-block.tool-running {
  border-left-color: var(--bc-flame);
}
.tool-block.tool-completed {
  border-left-color: var(--bc-success);
}
.tool-block.tool-error {
  border-left-color: var(--bc-error);
}
.tool-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.tool-name {
  font-family: var(--bc-font-mono);
  color: var(--bc-text);
  font-weight: 600;
}
.tool-status {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--bc-text-dim);
}
.tool-pulse {
  height: 2px;
  background: linear-gradient(90deg, var(--bc-flame), transparent);
  animation: pulse-slide 1.5s infinite;
  margin-top: var(--bc-space-sm);
}
@keyframes pulse-slide {
  0% { width: 0%; opacity: 1; }
  100% { width: 100%; opacity: 0.3; }
}
.tool-output pre {
  margin: var(--bc-space-sm) 0 0;
  font-size: 11px;
  color: var(--bc-text-muted);
  white-space: pre-wrap;
  font-family: var(--bc-font-mono);
}
.tool-error {
  color: var(--bc-error);
  margin-top: var(--bc-space-xs);
}
.permission-block {
  border: 1px solid var(--bc-flame);
  background: rgba(255, 154, 44, 0.05);
  padding: var(--bc-space-md);
  margin: var(--bc-space-sm) 0;
}
.perm-header {
  font-size: 12px;
  font-weight: 600;
  color: var(--bc-text);
  margin-bottom: var(--bc-space-xs);
}
.perm-type {
  font-size: 10px;
  text-transform: uppercase;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-sm);
}
.perm-actions {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
}
.perm-allow {
  border-color: var(--bc-success);
  color: var(--bc-success);
  background: transparent;
}
.perm-deny {
  border-color: var(--bc-error);
  color: var(--bc-error);
  background: transparent;
}
.perm-remember {
  margin-left: auto;
  font-size: 11px;
  color: var(--bc-text-dim);
  display: flex;
  align-items: center;
  gap: var(--bc-space-xs);
}
.perm-remember input {
  width: 14px;
  height: 14px;
  accent-color: var(--bc-core);
}
</style>
