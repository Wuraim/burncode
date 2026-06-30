<script setup lang="ts">
import { ref } from "vue";
import { useProvidersStore } from "../../stores/providers";
import type { ModelUsageStats } from "../../types/app";

const providers = useProvidersStore();

const showMore = ref(false);
const showAuth = ref(false);
const authProvider = ref<string>("");
const apiKey = ref("");
const message = ref<string | null>(null);
const expandedProvider = ref<string | null>(null);

function aggSummary(u: ModelUsageStats): string {
  if (u.requests === 0) return "No usage yet";
  const pct = u.requests > 0 ? Math.round((u.successes / u.requests) * 100) : 0;
  return `${u.requests} req · ${pct}% ok · $${u.total_cost.toFixed(3)} · ${(u.total_input_tokens + u.total_output_tokens + u.total_reasoning_tokens).toLocaleString()} tokens`;
}

function modelUsageSummary(u: ModelUsageStats | null): string {
  if (!u || u.requests === 0) return "never used";
  const pct = Math.round((u.successes / u.requests) * 100);
  const tokens = u.total_input_tokens + u.total_output_tokens + u.total_reasoning_tokens;
  return `${u.requests} req · ${pct}% · $${u.total_cost.toFixed(3)} · ${tokens.toLocaleString()} tok`;
}

function usagePercent(used: number, cap?: number) {
  if (!cap || cap === 0) return 0;
  return Math.min(100, Math.round((used / cap) * 100));
}

function openAuth(providerId: string) {
  authProvider.value = providerId;
  apiKey.value = "";
  message.value = null;
  showAuth.value = true;
}

async function submitAuth() {
  if (!authProvider.value || !apiKey.value) return;
  message.value = null;
  try {
    await providers.connectProvider(authProvider.value, apiKey.value);
    message.value = "Connected";
    showAuth.value = false;
  } catch (e) {
    message.value = String(e);
  }
}

function toggleModels(id: string) {
  expandedProvider.value = expandedProvider.value === id ? null : id;
}
</script>

<template>
  <div class="provider-panel">
    <div class="title">Providers</div>

    <ul class="list">
      <li v-for="p in providers.pinnedProviders" :key="p.id" class="row pinned" @click="toggleModels(p.id)">
        <div class="main">
          <span class="dot" :class="{ active: p.connected }"></span>
          <span class="name">{{ p.name }}</span>
          <button
            v-if="!p.connected"
            class="small"
            @click.stop="openAuth(p.id)"
          >
            Connect
          </button>
          <span v-else class="live-badge">LIVE</span>
        </div>

        <!-- Aggregate usage -->
        <div class="agg-usage">{{ aggSummary(p.aggregateUsage) }}</div>

        <!-- Quota bar -->
        <div class="meta">
          <span v-if="p.defaultModel" class="model">{{ p.defaultModel }}</span>
          <span class="source">{{ p.quota.source }}</span>
          <span v-if="p.quota.period_type" class="source period">{{ p.quota.period_type }}</span>
        </div>
        <div class="quota-bar-wrap">
          <div
            class="quota-bar"
            :style="{ width: usagePercent(p.quota.used_requests, p.quota.period_cap_requests || p.quota.requests_cap) + '%' }"
          ></div>
        </div>
        <div class="quota-meta">
          {{ p.quota.used_requests }}
          / {{ p.quota.period_cap_requests || p.quota.requests_cap || "∞" }} requests
        </div>

        <!-- Expandable model list -->
        <div v-if="expandedProvider === p.id && p.models.length > 0" class="model-list">
          <div v-for="m in p.models" :key="m.id" class="model-item" :class="{ unused: !m.usage || m.usage.requests === 0 }">
            <div class="model-name">{{ m.name }}</div>
            <div class="model-details">
              <span class="status" :class="m.status">{{ m.status }}</span>
              <span class="ctx">{{ m.limit?.context ? (m.limit.context / 1000).toFixed(0) + 'k' : '' }}</span>
            </div>
            <div class="model-usage">{{ modelUsageSummary(m.usage) }}</div>
          </div>
          <div v-if="p.models.length === 0" class="empty">No models loaded</div>
        </div>
      </li>
    </ul>

    <button class="toggle" @click="showMore = !showMore">
      {{ showMore ? "Hide others" : "More providers" }}
    </button>

    <ul v-if="showMore" class="list more">
      <li v-for="p in providers.otherProviders" :key="p.id" class="row">
        <span class="dot" :class="{ active: p.connected }"></span>
        <span class="name">{{ p.name }}</span>
        <button v-if="!p.connected" class="small" @click="openAuth(p.id)">Connect</button>
        <span v-else class="live-badge">LIVE</span>
      </li>
      <li v-if="providers.otherProviders.length === 0" class="empty">No other providers</li>
    </ul>

    <div v-if="showAuth" class="auth-form">
      <div class="auth-title">Connect {{ authProvider }}</div>
      <div class="field">
        <label>API key</label>
        <input v-model="apiKey" type="password" placeholder="API key" />
      </div>
      <button class="primary small" :disabled="!apiKey" @click="submitAuth">Submit</button>
      <div v-if="message" class="message">{{ message }}</div>
    </div>
  </div>
</template>

<style scoped>
.provider-panel {
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
.list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
.row {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
  font-size: 12px;
  color: var(--bc-text-muted);
}
.row.pinned {
  padding: var(--bc-space-sm);
  border: 1px solid var(--bc-border);
  background: var(--bc-panel-2);
  cursor: pointer;
}
.main {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
}
.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--bc-text-dim);
}
.dot.active {
  background: var(--bc-flame);
  box-shadow: 0 0 6px var(--bc-flame);
}
.name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 600;
  color: var(--bc-text);
}
.live-badge {
  font-size: 9px;
  padding: 1px 4px;
  border: 1px solid var(--bc-success);
  color: var(--bc-success);
  text-transform: uppercase;
}
.agg-usage {
  font-size: 10px;
  color: var(--bc-text-dim);
  font-family: var(--bc-font-mono);
  margin-top: var(--bc-space-xs);
}
.meta {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
  margin-top: var(--bc-space-xs);
}
.model {
  font-size: 10px;
  color: var(--bc-text-dim);
  font-family: var(--bc-font-mono);
}
.source {
  font-size: 9px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
  border: 1px solid var(--bc-border);
  padding: 0 3px;
}
.source.period {
  border-color: var(--bc-violet);
  color: var(--bc-violet);
}
button.small {
  font-size: 10px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
  background: transparent;
}
.toggle {
  width: 100%;
  margin-top: var(--bc-space-md);
  font-size: 11px;
  color: var(--bc-text-dim);
  background: transparent;
  border: 1px dashed var(--bc-border);
}
.more {
  margin-top: var(--bc-space-md);
  padding-top: var(--bc-space-md);
  border-top: 1px solid var(--bc-border);
}
.auth-form {
  margin-top: var(--bc-space-md);
  padding: var(--bc-space-md);
  border: 1px solid var(--bc-border);
  background: var(--bc-panel-2);
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
.auth-title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--bc-text-dim);
}
.field {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.field label {
  font-size: 10px;
}
.field input {
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  color: var(--bc-text);
  padding: var(--bc-space-sm);
  font-size: 12px;
}
.empty {
  color: var(--bc-text-dim);
  font-size: 12px;
}
.message {
  font-size: 11px;
  color: var(--bc-text-muted);
}
.quota-bar-wrap {
  height: 4px;
  background: var(--bc-border);
}
.quota-bar {
  height: 100%;
  background: linear-gradient(90deg, var(--bc-core), var(--bc-flame));
}
.quota-meta {
  font-size: 10px;
  color: var(--bc-text-dim);
  font-family: var(--bc-font-mono);
}
.model-list {
  margin-top: var(--bc-space-sm);
  padding: var(--bc-space-sm);
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.model-item {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  gap: var(--bc-space-xs);
}
.model-item.unused {
  opacity: 0.5;
}
.model-name {
  font-family: var(--bc-font-mono);
  color: var(--bc-text);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.model-details {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
}
.model-usage {
  font-size: 10px;
  color: var(--bc-text-dim);
  font-family: var(--bc-font-mono);
  width: 100%;
  margin-top: var(--bc-space-xs);
}
.status {
  font-size: 9px;
  text-transform: uppercase;
  padding: 1px 3px;
  border: 1px solid var(--bc-border);
}
.status.active {
  border-color: var(--bc-success);
  color: var(--bc-success);
}
.status.beta {
  border-color: var(--bc-flame);
  color: var(--bc-flame);
}
.ctx {
  font-size: 10px;
  color: var(--bc-text-dim);
}
</style>
