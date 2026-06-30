import { defineStore } from "pinia";
import { ref, reactive, computed } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invokeCommand } from "../services/tauri";
import type { OpencodeEvent, RouteDecision } from "../types/app";

export interface StreamingTextPart {
  id: string;
  type: "text";
  text: string;
  time?: { start?: number; end?: number };
}

export interface StreamingReasoningPart {
  id: string;
  type: "reasoning";
  text: string;
  collapsed: boolean;
}

export interface StreamingToolPart {
  id: string;
  type: "tool";
  callID: string;
  tool: string;
  status: "pending" | "running" | "completed" | "error";
  title?: string;
  input?: unknown;
  output?: string;
  error?: string;
}

export interface StreamingStepStartPart {
  id: string;
  type: "step-start";
}

export interface StreamingStepFinishPart {
  id: string;
  type: "step-finish";
  reason: string;
}

export type StreamingPart =
  | StreamingTextPart
  | StreamingReasoningPart
  | StreamingToolPart
  | StreamingStepStartPart
  | StreamingStepFinishPart;

export const useStreamingStore = defineStore("streaming", () => {
  const isStreaming = ref(false);
  const streamingSessionID = ref<string | null>(null);
  const sessionID = ref<string | null>(null);
  const messageID = ref<string | null>(null);
  const parts = reactive<Map<string, StreamingPart>>(new Map());
  const completionTimer = ref<number | null>(null);
  const seenPartIDs = reactive<Set<string>>(new Set());
  const currentDifficulty = ref<string>("unknown");
  const permissionRequests = reactive<PermissionRequest[]>([]);

  interface PermissionRequest {
    id: string;
    title: string;
    type: string;
    sessionID: string;
  }

  const orderedParts = computed(() => {
    return Array.from(parts.values());
  });

  const textParts = computed(() => {
    return orderedParts.value.filter((p): p is StreamingTextPart => p.type === "text");
  });

  const fullText = computed(() => {
    return textParts.value.map((p) => p.text).join("");
  });

  let unlisten: (() => void) | null = null;

  async function startListening() {
    if (unlisten) return;
    unlisten = await listen<OpencodeEvent>("opencode-event", (e) => {
      handleEvent(e.payload);
    });
  }

  function handleEvent(event: OpencodeEvent) {
    if (!isStreaming.value) return;

    // Filter events that don't belong to the session we're streaming for
    const data = event.data as { properties?: { sessionID?: string; info?: { sessionID?: string } } };
    const eventSessionID =
      data?.properties?.sessionID || data?.properties?.info?.sessionID;
    if (streamingSessionID.value && eventSessionID && eventSessionID !== streamingSessionID.value) {
      return;
    }

    if (event.event_type === "message.part.updated") {
      handlePartUpdated(data as unknown as { properties?: { part?: Record<string, unknown>; delta?: string } });
    } else if (event.event_type === "message.updated") {
      handleMessageUpdated(data as unknown as { properties?: { info?: Record<string, unknown> } });
    } else if (event.event_type === "permission.updated") {
      handlePermissionUpdated(data as unknown as { properties?: Record<string, unknown> });
    }
  }

  function handlePermissionUpdated(data: { properties?: Record<string, unknown> }) {
    const props = data?.properties;
    if (!props) return;

    // Check if this is a permission request (has title/type, not already replied)
    // The permission.updated event may fire for both new and replied permissions.
    // We only show new ones that haven't been replied yet.
    const replied = props.replied !== undefined;
    if (replied) return; // was already responded to

    const permissionId = props.id as string | undefined;
    if (!permissionId) return;

    // Check if we already have this permission
    if (permissionRequests.find((p) => p.id === permissionId)) return;

    permissionRequests.push({
      id: permissionId,
      title: (props.title as string) || "Permission requested",
      type: (props.type as string) || "unknown",
      sessionID: (props.sessionID as string) || "",
    });
  }

  function handlePartUpdated(data: { properties?: { part?: Record<string, unknown>; delta?: string } }) {
    const partRaw = data?.properties?.part;
    if (!partRaw) return;

    const partID = partRaw.id as string | undefined;
    if (!partID) return;

    const partType = partRaw.type as string | undefined;
    if (!partType) return;

    // Detect session/message ID
    if (partRaw.sessionID && !sessionID.value) sessionID.value = partRaw.sessionID as string;
    if (partRaw.messageID && !messageID.value) messageID.value = partRaw.messageID as string;

    // Accumulate text for text/reasoning parts
    const delta = data?.properties?.delta || "";

    if (partType === "text") {
      const existing = parts.get(partID) as StreamingTextPart | undefined;
      if (existing) {
        existing.text += delta;
        if (partRaw.text && typeof partRaw.text === "string" && partRaw.text.length > existing.text.length) {
          existing.text = partRaw.text;
        }
      } else {
        parts.set(partID, {
          id: partID,
          type: "text",
          text: (partRaw.text as string) || delta || "",
        });
      }
    } else if (partType === "reasoning") {
      const existing = parts.get(partID) as StreamingReasoningPart | undefined;
      if (existing) {
        existing.text += delta;
        if (partRaw.text && typeof partRaw.text === "string" && partRaw.text.length > existing.text.length) {
          existing.text = partRaw.text;
        }
      } else {
        parts.set(partID, {
          id: partID,
          type: "reasoning",
          text: (partRaw.text as string) || delta || "",
          collapsed: true,
        });
      }
    } else if (partType === "tool") {
      const stateRaw = partRaw.state as Record<string, unknown> | undefined;
      if (!stateRaw) return;

      const status = stateRaw.status as string | undefined;
      if (!status) return;

      const existing = parts.get(partID) as StreamingToolPart | undefined;
      if (existing) {
        existing.status = status as StreamingToolPart["status"];
        if (status === "running") {
          existing.title = stateRaw.title as string | undefined;
        } else if (status === "completed") {
          existing.title = stateRaw.title as string | undefined;
          existing.output = stateRaw.output as string | undefined;
        } else if (status === "error") {
          existing.error = stateRaw.error as string | undefined;
        }
      } else {
        parts.set(partID, {
          id: partID,
          type: "tool",
          callID: partRaw.callID as string || "",
          tool: partRaw.tool as string || "",
          status: status as StreamingToolPart["status"],
          title: stateRaw.title as string | undefined,
          input: stateRaw.input,
        });
      }
    } else if (partType === "step-start") {
      if (!parts.has(partID)) {
        parts.set(partID, { id: partID, type: "step-start" });
      }
    } else if (partType === "step-finish") {
      parts.set(partID, {
        id: partID,
        type: "step-finish",
        reason: (partRaw.reason as string) || "unknown",
      });
    }
  }

  function handleMessageUpdated(data: { properties?: { info?: Record<string, unknown> } }) {
    const info = data?.properties?.info;
    if (!info) return;
    const role = info.role as string | undefined;
    if (role !== "assistant") return;

    const completed = (info.time as Record<string, unknown> | undefined)?.completed;
    if (!completed) return;

    // Message is complete — record outcome and clean up
    const providerID = info.providerID as string | undefined;
    const modelID = info.modelID as string | undefined;
    const cost = (info.cost as number) || 0;
    const tokens = (info.tokens as Record<string, number>) || {};

    if (providerID && modelID) {
      invokeCommand("record_assistant_outcome", {
        providerId: providerID,
        modelId: modelID,
        cost,
        inputTokens: tokens.input || 0,
        outputTokens: tokens.output || 0,
        reasoningTokens: tokens.reasoning || 0,
        difficulty: currentDifficulty.value,
      }).catch(() => {});
    }

    // Auto-finalize after a short delay to allow final rendering
    if (completionTimer.value) clearTimeout(completionTimer.value);
    completionTimer.value = window.setTimeout(() => {
      finishStreaming();
    }, 500);
  }

  function classifyDifficulty(t: string): string {
    const chars = t.length;
    const words = t.split(/\s+/).length;
    if (chars > 4000 || words > 600) return "long_context";
    if (words > 200 || t.includes("refactor") || t.includes("architect")) return "deep";
    if (words > 60 || t.includes("implement") || t.includes("debug")) return "balanced";
    return "fast";
  }

  function startStreaming(sessionId: string, text: string, route: RouteDecision | null) {
    isStreaming.value = true;
    streamingSessionID.value = sessionId;
    sessionID.value = null;
    messageID.value = null;
    parts.clear();
    seenPartIDs.clear();
    currentDifficulty.value = route?.difficulty || classifyDifficulty(text);
    if (completionTimer.value) {
      clearTimeout(completionTimer.value);
      completionTimer.value = null;
    }
  }

  async function respondToPermission(permissionId: string, response: "allow" | "deny", remember?: boolean) {
    const perm = permissionRequests.find((p) => p.id === permissionId);
    if (!perm) return;

    try {
      await invokeCommand("respond_to_permission", {
        sessionId: perm.sessionID,
        permissionId: perm.id,
        response,
        remember: remember ?? false,
      });
    } catch {
      // Permission response failed silently
    }

    const idx = permissionRequests.indexOf(perm);
    if (idx >= 0) permissionRequests.splice(idx, 1);
  }

  function finishStreaming() {
    isStreaming.value = false;
    streamingSessionID.value = null;
    sessionID.value = null;
    messageID.value = null;
    parts.clear();
    seenPartIDs.clear();
    permissionRequests.splice(0);
    if (completionTimer.value) {
      clearTimeout(completionTimer.value);
      completionTimer.value = null;
    }
    // Trigger message reload from session store
    import("./session").then(({ useSessionStore }) => {
      useSessionStore().finalizeStreamingAndReload();
    });
  }

  return {
    isStreaming,
    streamingSessionID,
    sessionID,
    messageID,
    permissionRequests,
    orderedParts,
    textParts,
    fullText,
    startListening,
    startStreaming,
    finishStreaming,
    respondToPermission,
  };
});
