<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useAgentsStore } from "../../stores/agents";

const agents = useAgentsStore();

onMounted(() => {
  agents.loadAgents();
});

const primaryAgents = computed(() => {
  return agents.prioritizedAgents.filter((a) => a.name === "plan" || a.name === "build");
});

const secondaryAgents = computed(() => {
  return agents.agents.filter((a) => a.name !== "plan" && a.name !== "build");
});

function select(name: string) {
  agents.selectAgent(name);
}
</script>

<template>
  <div class="agent-selector">
    <button
      v-for="a in primaryAgents"
      :key="a.name"
      :class="['btn', { active: a.name === agents.selectedName }]"
      :title="a.description || a.name"
      @click="select(a.name)"
    >
      {{ a.name }}
    </button>
    <select
      v-if="secondaryAgents.length > 0"
      :value="agents.selectedName"
      class="more"
      @change="select(($event.target as HTMLSelectElement).value)"
    >
      <option value="" disabled>other</option>
      <option v-for="a in secondaryAgents" :key="a.name" :value="a.name">
        {{ a.name }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.agent-selector {
  display: flex;
  align-items: center;
  gap: var(--bc-space-xs);
}
.btn {
  padding: var(--bc-space-xs) var(--bc-space-sm);
  border: 1px solid var(--bc-border);
  background: var(--bc-panel-2);
  color: var(--bc-text-muted);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  border-radius: 0;
}
.btn.active {
  border-color: var(--bc-core);
  color: var(--bc-core);
  background: rgba(76, 201, 240, 0.08);
}
.more {
  background: transparent;
  border: 1px solid var(--bc-border);
  color: var(--bc-text-dim);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 10px;
  text-transform: uppercase;
}
</style>
