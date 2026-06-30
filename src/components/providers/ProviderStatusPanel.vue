<script setup lang="ts">
import { computed } from "vue";
import { useWorkspaceStore } from "../../stores/workspace";

const workspace = useWorkspaceStore();

const providerList = computed(() => {
  const data = workspace.providers as { all?: unknown[]; connected?: string[]; default?: Record<string, string> } | null;
  const all = (data?.all ?? []) as { id?: string; name?: string }[];
  const connected = new Set(data?.connected ?? []);
  return all.map((p) => ({
    id: p.id || "unknown",
    name: p.name || p.id || "unknown",
    connected: connected.has(p.id || ""),
  }));
});
</script>

<template>
  <div class="provider-panel">
    <div class="title">Providers</div>
    <ul class="list">
      <li v-for="p in providerList" :key="p.id" class="row">
        <span class="dot" :class="{ active: p.connected }"></span>
        <span class="name">{{ p.name }}</span>
        <span v-if="p.connected" class="badge">LIVE</span>
      </li>
      <li v-if="providerList.length === 0" class="empty">No providers loaded</li>
    </ul>
  </div>
</template>

<style scoped>
.provider-panel {
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
.list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-sm);
}
.row {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  font-size: 12px;
  color: var(--bc-text-muted);
}
.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--bc-text-dim);
}
.dot.active {
  background: var(--bc-flame);
  box-shadow: 0 0 6px var(--bc-flame);
}
.name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.badge {
  font-size: 9px;
  padding: 1px 4px;
  border: 1px solid var(--bc-flame);
  color: var(--bc-flame);
}
.empty {
  color: var(--bc-text-dim);
  font-size: 12px;
}
</style>
