<script setup lang="ts">
import { ref, computed } from "vue";
import { invokeCommand } from "../../services/tauri";
import { useWorkspaceStore } from "../../stores/workspace";

const workspace = useWorkspaceStore();
const authMethods = ref<Record<string, unknown>>({});
const selectedProvider = ref<string>("");
const credentialBody = ref<string>('{"key":""}');
const loading = ref(false);
const message = ref<string | null>(null);

const providers = computed(() => {
  return Object.keys(authMethods.value);
});

async function loadAuthMethods() {
  authMethods.value = await invokeCommand<Record<string, unknown>>("list_provider_auth_methods");
  if (providers.value.length > 0) {
    selectedProvider.value = providers.value[0];
  }
}

async function submitAuth() {
  if (!selectedProvider.value) return;
  loading.value = true;
  message.value = null;
  try {
    const body = JSON.parse(credentialBody.value);
    await invokeCommand("set_provider_auth", {
      providerId: selectedProvider.value,
      body,
    });
    message.value = "Credentials accepted";
    await workspace.loadProviders();
  } catch (e) {
    message.value = String(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="auth-panel">
    <div class="title flex-between">
      <span>Provider auth</span>
      <button class="small" @click="loadAuthMethods">Load</button>
    </div>
    <div v-if="providers.length > 0" class="form">
      <select v-model="selectedProvider">
        <option v-for="p in providers" :key="p" :value="p">{{ p }}</option>
      </select>
      <textarea v-model="credentialBody" rows="3" placeholder='{"key":"..."}' />
      <button class="primary small" :disabled="loading" @click="submitAuth">
        Submit
      </button>
    </div>
    <div v-if="message" class="message">{{ message }}</div>
  </div>
</template>

<style scoped>
.auth-panel {
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
.flex-between {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.form {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
select,
textarea {
  background: var(--bc-bg);
  border: 1px solid var(--bc-border);
  color: var(--bc-text);
  padding: var(--bc-space-sm);
  font-family: var(--bc-font-mono);
  font-size: 11px;
}
button.small {
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 11px;
}
.message {
  margin-top: var(--bc-space-sm);
  font-size: 11px;
  color: var(--bc-text-muted);
}
</style>
