import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { AppSettings, ConnectionProfile } from "../types/app";
import { invokeCommand } from "../services/tauri";

export const useConnectionStore = defineStore("connection", () => {
  const settings = ref<AppSettings>({});
  const connected = ref(false);
  const connecting = ref(false);
  const error = ref<string | null>(null);
  const serverVersion = ref<string | null>(null);

  const profile = computed(() => settings.value.profile);

  async function loadSettings() {
    settings.value = await invokeCommand<AppSettings>("load_settings");
    connected.value = await invokeCommand<boolean>("is_connected");
  }

  async function saveSettings(payload: AppSettings) {
    await invokeCommand("save_settings", { settings: payload });
    settings.value = payload;
  }

  async function connect(payload: ConnectionProfile) {
    connecting.value = true;
    error.value = null;
    try {
      const health = await invokeCommand<{ version: string }>("connect", { profile: payload });
      connected.value = true;
      serverVersion.value = health.version;
      settings.value = { ...settings.value, profile: payload };
    } catch (e) {
      connected.value = false;
      error.value = String(e);
      throw e;
    } finally {
      connecting.value = false;
    }
  }

  async function disconnect() {
    await invokeCommand("disconnect");
    connected.value = false;
    serverVersion.value = null;
  }

  return {
    settings,
    connected,
    connecting,
    error,
    serverVersion,
    profile,
    loadSettings,
    saveSettings,
    connect,
    disconnect,
  };
});
