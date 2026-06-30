import { defineStore } from "pinia";
import { ref } from "vue";
import { invokeCommand } from "../services/tauri";
import { useStreamingStore } from "./streaming";
import type { RouteDecision } from "../types/app";

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

  async function sendPrompt(text: string, agent?: string) {
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
        agent: agent || null,
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

  async function sendPromptAsync(text: string, agent?: string) {
    const ses = currentSession.value as { id?: string } | null;
    if (!ses?.id) return;
    sending.value = true;
    error.value = null;

    const streaming = useStreamingStore();

    try {
      await suggestRoute(text);
      streaming.startStreaming(ses.id, text, route.value);
      await invokeCommand("send_prompt_async", {
        id: ses.id,
        text,
        provider_id: route.value?.provider_id || null,
        model_id: route.value?.model_id || null,
        agent: agent || null,
      });
      // Streaming will be finalized by the SSE handler via streaming store
    } catch (e) {
      streaming.finishStreaming();
      error.value = String(e);
      throw e;
    } finally {
      sending.value = false;
    }
  }

  async function forkSession(sessionId: string, messageId?: string) {
    const created = await invokeCommand("fork_session", {
      id: sessionId,
      messageId: messageId || null,
    });
    const { useWorkspaceStore } = await import("./workspace");
    await useWorkspaceStore().loadSessions();
    const newId = (created as { id?: string }).id;
    if (newId) await openSession(newId);
  }

  async function abortSession(id: string) {
    await invokeCommand("abort_session", { id });
  }

  async function finalizeStreamingAndReload() {
    const ses = currentSession.value as { id?: string } | null;
    if (!ses?.id) return;
    await Promise.all([loadMessages(ses.id), loadTodos(ses.id), loadDiff(ses.id)]);
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
    sendPromptAsync,
    suggestRoute,
    abortSession,
    forkSession,
    finalizeStreamingAndReload,
    loadTodos,
    loadDiff,
  };
});
