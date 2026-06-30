<script setup lang="ts">
import { computed, ref } from "vue";
import { useSessionStore } from "../../stores/session";

const session = useSessionStore();
const selected = ref<unknown>(null);
const expanded = ref(false);

function filePath(d: unknown): string {
  return (d as { file?: string }).file || "unknown";
}

function fileDiff(d: unknown): string {
  const item = d as {
    file?: string;
    before?: string;
    after?: string;
    additions?: number;
    deletions?: number;
  };
  const lines: string[] = [];
  if (item.file) lines.push(`--- ${item.file}`);
  lines.push(`additions: ${item.additions ?? 0}  deletions: ${item.deletions ?? 0}`);
  if (item.before) {
    lines.push("\n[before]");
    lines.push(item.before.slice(0, 1200));
  }
  if (item.after) {
    lines.push("\n[after]");
    lines.push(item.after.slice(0, 1200));
  }
  return lines.join("\n");
}

const files = computed(() => session.diff);
</script>

<template>
  <div class="diff-panel" :class="{ collapsed: !expanded }">
    <div class="title" @click="expanded = !expanded">
      <span>{{ expanded ? "▾" : "▸" }}</span>
      <span>Diff inspector</span>
      <span class="count">({{ files.length }})</span>
    </div>
    <div v-show="expanded" class="body">
      <ul class="file-list">
        <li
          v-for="d in files"
          :key="filePath(d)"
          :class="['file', { active: selected === d }]"
          @click="selected = d"
        >
          {{ filePath(d) }}
        </li>
        <li v-if="files.length === 0" class="empty">No diff available</li>
      </ul>
      <div class="viewer">
        <pre v-if="selected" class="patch">{{ fileDiff(selected) }}</pre>
        <div v-else class="empty-state">Select a file to inspect</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.diff-panel {
  border-top: 1px solid var(--bc-border);
  background: var(--bc-panel);
  display: flex;
  flex-direction: column;
}
.diff-panel.collapsed {
  min-height: 0;
  height: auto;
}
.title {
  padding: var(--bc-space-sm) var(--bc-space-md);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--bc-text-dim);
  border-bottom: 1px solid var(--bc-border);
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
}
.title:hover {
  background: var(--bc-panel-2);
}
.count {
  color: var(--bc-text-dim);
  font-size: 10px;
}
.body {
  display: flex;
  overflow: hidden;
  max-height: 280px;
}
.file-list {
  width: 200px;
  min-width: 200px;
  list-style: none;
  margin: 0;
  padding: var(--bc-space-sm) 0;
  border-right: 1px solid var(--bc-border);
  overflow-y: auto;
}
.file {
  padding: var(--bc-space-sm) var(--bc-space-md);
  cursor: pointer;
  color: var(--bc-text-muted);
  font-size: 12px;
  font-family: var(--bc-font-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.file:hover {
  background: var(--bc-panel-2);
}
.file.active {
  background: rgba(76, 201, 240, 0.08);
  color: var(--bc-core);
}
.viewer {
  flex: 1;
  overflow: auto;
  padding: var(--bc-space-md);
}
.patch {
  margin: 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--bc-text-muted);
  white-space: pre-wrap;
}
.empty,
.empty-state {
  color: var(--bc-text-dim);
  font-size: 12px;
  padding: var(--bc-space-md);
}
</style>
