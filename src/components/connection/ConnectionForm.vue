<script setup lang="ts">
import { ref, watch } from "vue";
import type { ConnectionProfile } from "../../types/app";

const props = defineProps<{ modelValue?: ConnectionProfile }>();
const emit = defineEmits<{
  submit: [profile: ConnectionProfile];
}>();

const profile = ref<ConnectionProfile>({
  server_url: "http://127.0.0.1:4096",
  username: "opencode",
  password: "",
  auto_connect: false,
});

watch(
  () => props.modelValue,
  (v) => {
    if (v) profile.value = { ...v };
  },
  { immediate: true }
);

function submit() {
  emit("submit", { ...profile.value });
}
</script>

<template>
  <form class="connection-form" @submit.prevent="submit">
    <div class="field">
      <label>Server URL</label>
      <input v-model="profile.server_url" type="url" required placeholder="http://127.0.0.1:4096" />
    </div>
    <div class="field">
      <label>Username</label>
      <input v-model="profile.username" type="text" placeholder="opencode" />
    </div>
    <div class="field">
      <label>Password</label>
      <input v-model="profile.password" type="password" placeholder="••••••••" />
    </div>
    <label class="check">
      <input v-model="profile.auto_connect" type="checkbox" />
      <span>Auto-connect on launch</span>
    </label>
    <button type="submit" class="primary">Ignite connection</button>
  </form>
</template>

<style scoped>
.connection-form {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-md);
}
.field {
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.check {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  color: var(--bc-text-muted);
  text-transform: none;
  font-size: 12px;
  letter-spacing: normal;
}
.check input {
  width: 16px;
  height: 16px;
  accent-color: var(--bc-core);
}
button {
  margin-top: var(--bc-space-sm);
}
</style>
