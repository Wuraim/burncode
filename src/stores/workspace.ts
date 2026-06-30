import { defineStore } from "pinia";
import { ref } from "vue";
import { invokeCommand } from "../services/tauri";

export const useWorkspaceStore = defineStore("workspace", () => {
  const projects = ref<unknown[]>([]);
  const currentProject = ref<unknown>(null);
  const sessions = ref<unknown[]>([]);
  const providers = ref<unknown>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadProjects() {
    projects.value = (await invokeCommand<unknown[]>("list_projects")) ?? [];
  }

  async function loadCurrentProject() {
    currentProject.value = await invokeCommand<unknown>("current_project");
  }

  async function loadProviders() {
    providers.value = await invokeCommand<unknown>("list_providers");
  }

  async function loadSessions() {
    sessions.value = (await invokeCommand<unknown[]>("list_sessions")) ?? [];
  }

  async function createSession(title?: string) {
    const session = await invokeCommand<unknown>("create_session", { title: title || undefined });
    await loadSessions();
    return session;
  }

  async function loadAll() {
    loading.value = true;
    error.value = null;
    try {
      await Promise.all([loadProjects(), loadCurrentProject(), loadProviders(), loadSessions()]);
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  return {
    projects,
    currentProject,
    sessions,
    providers,
    loading,
    error,
    loadProjects,
    loadCurrentProject,
    loadProviders,
    loadSessions,
    createSession,
    loadAll,
  };
});
