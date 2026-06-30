import { defineStore } from "pinia";
import { ref } from "vue";
import { invokeCommand } from "../services/tauri";

export interface RouteDecision {
  provider_id: string;
  model_id: string;
  difficulty: string;
  reason: string;
}

export const useSessionStore = defineStore("session", () => {
  const currentSession = ref<unknown>(null);
  const messages = ref<unknown[]>([]);
  const todos = ref<unknown[]>([]);
  const diff = ref<unknown[]>([]);
  const sending = ref(false);
  const error = ref<string | null>(null);
  const route = ref<RouteDecision | null>(null);

  async function openSession(id: string) {
    currentSession.value = await invokeCommand<unknown>("get_session", { id });
    await Promise.all([loadMessages(id), loadTodos(id), loadDiff(id)]);
  }

  async function suggestRoute(text: string) {
    route.value = await invokeCommand<RouteDecision | null>("suggest_route", { text });
  }

  async function loadMessages(id: string) {
    messages.value = (await invokeCommand<unknown[]>("get_session_messages", { id })) ?? [];
  }

  async function loadTodos(id: string) {
    todos.value = (await invokeCommand<unknown[]>("get_todos", { id })) ?? [];
  }

  async function loadDiff(id: string) {
    diff.value = (await invokeCommand<unknown[]>("get_diff", { id })) ?? [];
  }

  async function sendPrompt(text: string) {
    const session = currentSession.value as { id?: string } | null;
    if (!session?.id) return;
    sending.value = true;
    error.value = null;
    try {
      await suggestRoute(text);
      await invokeCommand("send_prompt", {
        id: session.id,
        text,
        provider_id: route.value?.provider_id || null,
        model_id: route.value?.model_id || null,
      });
      await loadMessages(session.id);
      await loadTodos(session.id);
      await loadDiff(session.id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      sending.value = false;
    }
  }

  return {
    currentSession,
    messages,
    todos,
    diff,
    sending,
    error,
    route,
    openSession,
    sendPrompt,
    suggestRoute,
    loadTodos,
    loadDiff,
  };
});
