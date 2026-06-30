<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useSessionStore } from "../../stores/session";

const session = useSessionStore();
const burnLevel = ref<"low" | "medium" | "high" | null>(null);
const seenIds = new Set<string>();

interface ModelUsage {
  key: string;
  provider: string;
  model: string;
  count: number;
}

function messageId(m: unknown): string | undefined {
  return (m as { info?: { id?: string } }).info?.id;
}

function totalTokens(m: unknown): number {
  const tokens = (m as { info?: { tokens?: { input?: number; output?: number; reasoning?: number } } }).info?.tokens;
  if (!tokens) return 0;
  return (tokens.input || 0) + (tokens.output || 0) + (tokens.reasoning || 0);
}

watch(
  () => session.messages,
  (messages) => {
    let maxBurn = 0;
    for (const m of messages) {
      const info = (m as { info?: { role?: string } }).info;
      if (info?.role !== "assistant") continue;
      const id = messageId(m);
      if (id && seenIds.has(id)) continue;
      if (id) seenIds.add(id);
      const tokens = totalTokens(m);
      if (tokens >= 10000) maxBurn = Math.max(maxBurn, 3);
      else if (tokens >= 5000) maxBurn = Math.max(maxBurn, 2);
      else if (tokens >= 1000) maxBurn = Math.max(maxBurn, 1);
    }
    if (maxBurn === 0) return;
    burnLevel.value = maxBurn === 3 ? "high" : maxBurn === 2 ? "medium" : "low";
    setTimeout(() => {
      burnLevel.value = null;
    }, 1200);
  },
  { deep: true }
);

const usage = computed<ModelUsage[]>(() => {
  const counts = new Map<string, ModelUsage>();
  for (const entry of session.messages) {
    const info = (entry as { info?: { role?: string; providerID?: string; modelID?: string } }).info;
    if (info?.role !== "assistant") continue;
    const provider = info.providerID || "unknown";
    const model = info.modelID || "unknown";
    const key = `${provider}/${model}`;
    const existing = counts.get(key);
    if (existing) {
      existing.count++;
    } else {
      counts.set(key, { key, provider, model, count: 1 });
    }
  }
  return Array.from(counts.values()).sort((a, b) => b.count - a.count);
});

const total = computed(() => usage.value.reduce((sum, u) => sum + u.count, 0));

const mostUsed = computed(() => usage.value[0] || null);

const topThree = computed(() => usage.value.slice(0, 3));

function share(count: number) {
  if (total.value === 0) return "0%";
  return `${Math.round((count / total.value) * 100)}%`;
}
</script>

<template>
  <div class="telemetry-panel">
    <div class="title">Telemetry</div>
    <div v-if="mostUsed" class="hero" :class="['burn', burnLevel]">
      <div class="hero-label">Most used model</div>
      <div class="hero-model">{{ mostUsed.provider }} / {{ mostUsed.model }}</div>
      <div class="hero-meta">{{ mostUsed.count }} response{{ mostUsed.count === 1 ? "" : "s" }} · {{ share(mostUsed.count) }}</div>
    </div>
    <div v-else class="hero empty">No assistant responses yet</div>
    <ul v-if="topThree.length > 1" class="ranking">
      <li v-for="u in topThree" :key="u.key" class="row">
        <span class="name">{{ u.provider }} / {{ u.model }}</span>
        <span class="bar-wrap">
          <span class="bar" :style="{ width: share(u.count) }"></span>
        </span>
        <span class="count">{{ u.count }}</span>
      </li>
    </ul>
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
.hero {
  padding: var(--bc-space-md);
  border: 1px solid var(--bc-violet);
  background: rgba(123, 97, 255, 0.08);
  margin-bottom: var(--bc-space-md);
  transition: box-shadow 0.2s, border-color 0.2s, background 0.2s;
}
.hero.empty {
  border-color: var(--bc-border);
  background: transparent;
  color: var(--bc-text-dim);
  font-size: 12px;
}
.hero.burn.low {
  border-color: var(--bc-core);
  box-shadow: 0 0 12px rgba(76, 201, 240, 0.35);
}
.hero.burn.medium {
  border-color: var(--bc-magenta);
  box-shadow: 0 0 18px rgba(217, 70, 239, 0.45);
}
.hero.burn.high {
  border-color: var(--bc-flame);
  background: rgba(255, 90, 54, 0.12);
  box-shadow: 0 0 28px rgba(255, 90, 54, 0.55);
}
.hero-label {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-xs);
}
.hero-model {
  font-size: 16px;
  font-weight: 700;
  color: var(--bc-core);
  font-family: var(--bc-font-mono);
  word-break: break-all;
}
.hero-meta {
  margin-top: var(--bc-space-xs);
  font-size: 11px;
  color: var(--bc-text-muted);
}
.ranking {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
.row {
  display: grid;
  grid-template-columns: 1fr 60px 24px;
  align-items: center;
  gap: var(--bc-space-sm);
  font-size: 11px;
  color: var(--bc-text-muted);
}
.name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.bar-wrap {
  height: 4px;
  background: var(--bc-border);
}
.bar {
  display: block;
  height: 100%;
  background: linear-gradient(90deg, var(--bc-core), var(--bc-violet));
}
.count {
  text-align: right;
  font-family: var(--bc-font-mono);
}
</style>
