<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invokeCommand } from "../../services/tauri";
import { useWorkspaceStore } from "../../stores/workspace";
import type { ProviderAuthMethod, RouteConfig, ProviderQuotaConfig } from "../../types/app";

const workspace = useWorkspaceStore();
const PINNED = ["openai", "opencode"];

const showMore = ref(false);
const showAuth = ref(false);
const authProvider = ref<string>("");
const authMethods = ref<Record<string, ProviderAuthMethod[]>>({});
const apiKey = ref("");
const loading = ref(false);
const message = ref<string | null>(null);
const routeConfig = ref<RouteConfig | null>(null);

onMounted(() => {
  loadRouteConfig();
});

async function loadRouteConfig() {
  try {
    routeConfig.value = await invokeCommand<RouteConfig>("get_route_config");
  } catch {
    // ignore; panel still works without analytics
  }
}

function quota(providerId: string): ProviderQuotaConfig {
  return (
    (routeConfig.value?.analytics.provider_quotas as Record<string, ProviderQuotaConfig> | undefined)?.[
      providerId
    ] ?? {
      source: "estimated",
      used_requests: 0,
      used_budget: 0,
      used_tokens: 0,
    }
  );
}

function usagePercent(used: number, cap?: number) {
  if (!cap || cap === 0) return 0;
  return Math.min(100, Math.round((used / cap) * 100));
}

const providers = computed(() => {
  const data = workspace.providers as { all?: unknown[]; connected?: string[]; default?: Record<string, string> } | null;
  const all = (data?.all ?? []) as { id?: string; name?: string }[];
  const connected = new Set(data?.connected ?? []);
  const defaults = data?.default ?? {};
  return all.map((p) => ({
    id: p.id || "unknown",
    name: p.name || p.id || "unknown",
    connected: connected.has(p.id || ""),
    defaultModel: (defaults as Record<string, string>)[p.id || ""] || "",
  }));
});

const pinned = computed(() => providers.value.filter((p) => PINNED.includes(p.id)));
const others = computed(() => providers.value.filter((p) => !PINNED.includes(p.id)));

function openAuth(providerId: string) {
  authProvider.value = providerId;
  apiKey.value = "";
  message.value = null;
  showAuth.value = true;
  loadAuthMethods();
}

async function loadAuthMethods() {
  if (!authProvider.value) return;
  try {
    const methods = await invokeCommand<Record<string, ProviderAuthMethod[]>>("list_provider_auth_methods");
    authMethods.value = methods;
  } catch (e) {
    message.value = String(e);
  }
}

const currentMethods = computed(() => {
  return authMethods.value[authProvider.value] ?? [];
});

const apiMethod = computed(() => currentMethods.value.find((m) => m.type === "api"));

async function submitAuth() {
  if (!authProvider.value || !apiMethod.value) return;
  loading.value = true;
  message.value = null;
  try {
    await invokeCommand("set_provider_auth", {
      providerId: authProvider.value,
      body: { type: "api", key: apiKey.value },
    });
    message.value = "Connected";
    await workspace.loadProviders();
  } catch (e) {
    message.value = String(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="provider-panel">
    <div class="title">Providers</div>

    <ul class="list">
      <li v-for="p in pinned" :key="p.id" class="row pinned">
        <div class="main">
          <span class="dot" :class="{ active: p.connected }"></span>
          <span class="name">{{ p.name }}</span>
          <button class="small" @click="openAuth(p.id)">{{ p.connected ? "Manage" : "Connect" }}</button>
        </div>
        <div class="meta">
          <span v-if="p.defaultModel" class="model">{{ p.defaultModel }}</span>
          <span class="source">{{ quota(p.id).source }}</span>
        </div>
        <div class="quota-bar-wrap">
          <div
            class="quota-bar"
            :style="{ width: usagePercent(quota(p.id).used_requests, quota(p.id).requests_cap) + '%' }"
          ></div>
        </div>
        <div class="quota-meta">
          {{ quota(p.id).used_requests }} / {{ quota(p.id).requests_cap ?? "∞" }} requests
        </div>
      </li>
    </ul>

    <button class="toggle" @click="showMore = !showMore">
      {{ showMore ? "Hide others" : "More providers" }}
    </button>

    <ul v-if="showMore" class="list more">
      <li v-for="p in others" :key="p.id" class="row">
        <span class="dot" :class="{ active: p.connected }"></span>
        <span class="name">{{ p.name }}</span>
        <button class="small" @click="openAuth(p.id)">{{ p.connected ? "Manage" : "Connect" }}</button>
      </li>
      <li v-if="others.length === 0" class="empty">No other providers</li>
    </ul>

    <div v-if="showAuth" class="auth-form">
      <div class="auth-title">Connect {{ authProvider }}</div>
      <div v-if="apiMethod" class="field">
        <label>{{ apiMethod.label }}</label>
        <input v-model="apiKey" type="password" placeholder="API key" />
      </div>
      <div v-else-if="currentMethods.length > 0" class="methods">
        <div v-for="m in currentMethods" :key="m.type" class="method">{{ m.label }}</div>
      </div>
      <div v-else class="empty">No auth method loaded</div>
      <button v-if="apiMethod" class="primary small" :disabled="loading || !apiKey" @click="submitAuth">
        Submit
      </button>
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
}
.meta {
  display: flex;
  gap: var(--bc-space-sm);
  align-items: center;
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
.methods {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.method {
  font-size: 12px;
  color: var(--bc-text-muted);
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
</style>
