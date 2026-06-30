<script setup lang="ts">
import { ref, computed } from "vue";
import { invokeCommand } from "../../services/tauri";
import { useSessionStore } from "../../stores/session";
import { useProvidersStore } from "../../stores/providers";
import type { RouteConfig } from "../../types/app";

const session = useSessionStore();
const providersStore = useProvidersStore();
const open = ref(false);
const config = ref<RouteConfig | null>(null);
const loading = ref(false);

const pinnedIds = ["openai", "opencode"];

const connectedProviders = computed(() => {
  const data = config.value?.providers as { connected?: string[]; default?: Record<string, string> } | undefined;
  const all = data?.connected ?? [];
  const defaults = data?.default ?? {};
  const filtered = all.filter((id) => pinnedIds.includes(id));
  return {
    connected: filtered,
    defaults,
  };
});

const modelKeys = computed(() => {
  const all = Object.keys(config.value?.analytics.models ?? {});
  return all.filter((key) => pinnedIds.some((pid) => key.startsWith(pid + "/")));
});

async function toggle() {
  open.value = !open.value;
  if (open.value && !config.value && !loading.value) {
    await loadConfig();
  }
}

async function loadConfig() {
  loading.value = true;
  try {
    config.value = await invokeCommand<RouteConfig>("get_route_config");
    await providersStore.refresh();
  } finally {
    loading.value = false;
  }
}

async function setQuota(
  providerId: string,
  requestsCap: number | undefined,
  periodType?: "daily" | "monthly"
) {
  await providersStore.setQuota(providerId, requestsCap, undefined, undefined, periodType);
  await loadConfig();
}

function usagePercent(used: number, cap?: number) {
  if (!cap || cap === 0) return 0;
  return Math.min(100, Math.round((used / cap) * 100));
}
</script>

<template>
  <div class="route-config">
    <button class="trigger" @click="toggle">Route config</button>
    <div v-if="open" class="dropdown">
      <div class="header">
        <span>Auto-route config</span>
        <button class="small" @click="open = false">Close</button>
      </div>
      <div v-if="loading" class="body">Loading…</div>
      <div v-else-if="config" class="body">
        <div class="section">
          <div class="title">Selection rule</div>
          <div class="text">{{ config.policy.selection_rule }}</div>
        </div>

        <div class="section">
          <div class="title">Difficulty buckets</div>
          <ul class="list">
            <li v-for="b in config.policy.buckets" :key="b.name">
              <span class="badge">{{ b.name }}</span>
              <span class="desc">{{ b.description }}</span>
            </li>
          </ul>
        </div>

        <div class="section">
          <div class="title">Active providers</div>
          <div v-if="connectedProviders.connected.length === 0" class="text dim">None connected</div>
          <ul v-else class="list">
            <li v-for="p in connectedProviders.connected" :key="p">
              <span class="badge">{{ p }}</span>
              <span class="desc">{{ connectedProviders.defaults[p] || "no default" }}</span>
            </li>
          </ul>
        </div>

        <div class="section">
          <div class="title">Pinned provider quotas</div>
          <div v-for="p in providersStore.pinnedProviders" :key="p.id" class="quota-row">
            <div class="quota-header">
              <span class="badge">{{ p.id }}</span>
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
              {{ p.quota.used_requests }} / {{ p.quota.period_cap_requests || p.quota.requests_cap || "∞" }} requests
            </div>
            <div class="quota-input">
              <input
                type="number"
                placeholder="daily/monthly cap"
                :value="p.quota.requests_cap"
                @change="setQuota(p.id, ($event.target as HTMLInputElement).valueAsNumber || undefined, undefined)"
              />
            </div>
            <div class="quota-period">
              <label>Period</label>
              <select
                :value="p.quota.period_type || ''"
                @change="setQuota(p.id, p.quota.requests_cap, ($event.target as HTMLSelectElement).value as 'daily' | 'monthly' | undefined)"
              >
                <option value="">none</option>
                <option value="daily">daily</option>
                <option value="monthly">monthly</option>
              </select>
            </div>
          </div>
        </div>

        <div v-if="modelKeys.length > 0" class="section">
          <div class="title">Learned model usage</div>
          <ul class="compact-list">
            <li v-for="key in modelKeys" :key="key" class="row">
              <span class="name">{{ key }}</span>
              <span class="value">
                {{ config.analytics.models[key].successes }}/{{ config.analytics.models[key].requests }} ok ·
                ${{ config.analytics.models[key].total_cost.toFixed(4) }}
              </span>
            </li>
          </ul>
        </div>

        <div v-if="session.route" class="section">
          <div class="title">Last route</div>
          <div class="row">
            <span class="label">Model</span>
            <span class="value">{{ session.route.provider_id }} / {{ session.route.model_id }}</span>
          </div>
          <div class="row">
            <span class="label">Difficulty</span>
            <span class="value">{{ session.route.difficulty }}</span>
          </div>
          <div class="row">
            <span class="label">Source</span>
            <span class="value">{{ session.route.source }}</span>
          </div>
          <div class="row">
            <span class="label">Reason</span>
            <span class="value">{{ session.route.reason }}</span>
          </div>
          <div v-if="session.route.candidates.length > 0" class="candidates">
            <div class="title">Alternatives</div>
            <div v-for="c in session.route.candidates" :key="`${c.provider_id}/${c.model_id}`" class="row">
              <span class="label">{{ c.provider_id }} / {{ c.model_id }}</span>
              <span class="value">{{ Math.round(c.score) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.route-config {
  position: relative;
}
.trigger {
  font-size: 12px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
  background: var(--bc-panel-2);
  border: 1px solid var(--bc-border);
}
.dropdown {
  position: absolute;
  top: calc(100% + var(--bc-space-xs));
  right: 0;
  min-width: 360px;
  max-width: 420px;
  max-height: 560px;
  overflow-y: auto;
  background: var(--bc-panel);
  border: 1px solid var(--bc-border);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
  z-index: 100;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--bc-space-sm) var(--bc-space-md);
  border-bottom: 1px solid var(--bc-border);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--bc-text-dim);
}
.body {
  padding: var(--bc-space-md);
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-lg);
}
.section .title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-sm);
}
.row {
  display: flex;
  justify-content: space-between;
  gap: var(--bc-space-md);
  font-size: 12px;
  padding: var(--bc-space-xs) 0;
  border-bottom: 1px solid var(--bc-border);
}
.row .label {
  color: var(--bc-text-muted);
}
.row .value {
  color: var(--bc-core);
  font-family: var(--bc-font-mono);
  text-align: right;
}
.text {
  font-size: 12px;
  color: var(--bc-text-muted);
  line-height: 1.5;
}
.text.dim {
  color: var(--bc-text-dim);
}
.list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.list li {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  font-size: 12px;
}
.compact-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.compact-list .row {
  font-size: 11px;
}
.badge {
  padding: 1px 4px;
  border: 1px solid var(--bc-violet);
  color: var(--bc-violet);
  font-size: 10px;
  text-transform: uppercase;
}
.desc {
  color: var(--bc-text-muted);
}
button.small {
  font-size: 11px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
}
.quota-row {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
  margin-bottom: var(--bc-space-md);
  padding: var(--bc-space-sm);
  border: 1px solid var(--bc-border);
  background: var(--bc-panel-2);
}
.quota-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.source {
  font-size: 10px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
}
.source.period {
  border-color: var(--bc-violet);
  color: var(--bc-violet);
  border: 1px solid var(--bc-border);
  padding: 0 3px;
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
  font-size: 11px;
  color: var(--bc-text-muted);
  font-family: var(--bc-font-mono);
}
.quota-period {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
  margin-top: var(--bc-space-xs);
}
.quota-period label {
  font-size: 10px;
  color: var(--bc-text-dim);
}
.quota-period select {
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  color: var(--bc-text);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 11px;
}
.quota-input input {
  width: 100%;
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  color: var(--bc-text);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 12px;
}
.candidates {
  margin-top: var(--bc-space-md);
  padding-top: var(--bc-space-md);
  border-top: 1px solid var(--bc-border);
}
</style>
