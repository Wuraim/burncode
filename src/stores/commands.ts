import { defineStore } from "pinia";
import { ref } from "vue";
import { invokeCommand } from "../services/tauri";
import { useSessionStore } from "./session";

export interface Command {
  name: string;
  description?: string;
  template?: string;
  agent?: string;
  model?: string;
  subtask?: boolean;
}

export const useCommandsStore = defineStore("commands", () => {
  const commands = ref<Command[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function listCommands() {
    loading.value = true;
    error.value = null;
    try {
      const list = (await invokeCommand<Command[]>("list_commands")) ?? [];
      commands.value = list;
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function executeCommand(cmd: string, args?: string, agent?: string) {
    const ses = useSessionStore().currentSession as { id?: string } | null;
    if (!ses?.id) return;
    await invokeCommand("execute_command", {
      id: ses.id,
      command: cmd,
      arguments: args || null,
      agent: agent || null,
      modelId: null,
      providerId: null,
    });
  }

  return {
    commands,
    loading,
    error,
    listCommands,
    executeCommand,
  };
});
