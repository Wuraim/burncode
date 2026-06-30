import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invokeCommand } from "../services/tauri";
import type { Agent } from "../types/app";

export const useAgentsStore = defineStore("agents", () => {
  const agents = ref<Agent[]>([]);
  const selectedName = ref<string>("");
  const loading = ref(false);
  const error = ref<string | null>(null);

  const selectedAgent = computed(() => {
    return agents.value.find((a) => a.name === selectedName.value) || null;
  });

  const prioritizedAgents = computed(() => {
    const order = ["plan", "build"];
    const sorted = [...agents.value];
    sorted.sort((a, b) => {
      const ai = order.indexOf(a.name);
      const bi = order.indexOf(b.name);
      if (ai !== -1 && bi !== -1) return ai - bi;
      if (ai !== -1) return -1;
      if (bi !== -1) return 1;
      return (a.name || "").localeCompare(b.name || "");
    });
    return sorted;
  });

  async function loadAgents() {
    if (agents.value.length > 0) return;
    loading.value = true;
    error.value = null;
    try {
      const list = (await invokeCommand<Agent[]>("list_agents")) ?? [];
      agents.value = list;
      if (!selectedName.value && list.length > 0) {
        const plan = list.find((a) => a.name === "plan");
        selectedName.value = plan?.name || list[0].name;
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  function selectAgent(name: string) {
    selectedName.value = name;
  }

  return {
    agents,
    selectedName,
    selectedAgent,
    prioritizedAgents,
    loading,
    error,
    loadAgents,
    selectAgent,
  };
});
