<script setup lang="ts">
import { computed } from "vue";
import { useWorkspaceStore } from "../../stores/workspace";
import { useSessionStore } from "../../stores/session";

const workspace = useWorkspaceStore();
const session = useSessionStore();

const currentProjectName = computed(() => {
  const current = workspace.currentProject as { worktree?: string; id?: string } | null;
  if (current?.worktree) return current.worktree;
  const first = workspace.projects[0] as { worktree?: string; id?: string } | undefined;
  return first?.worktree || first?.id || "Unknown project";
});

function selectSession(id: string) {
  session.openSession(id);
}

function sessionTitle(s: unknown) {
  const item = s as { id?: string; title?: string; directory?: string };
  return item.title || item.directory || item.id || "Session";
}

function sessionId(s: unknown) {
  return (s as { id?: string }).id || "";
}

const currentId = computed(() => {
  const s = session.currentSession as { id?: string } | null;
  return s?.id || "";
});

async function newSession() {
  await workspace.createSession("New session");
  const first = workspace.sessions[0];
  if (first) selectSession(sessionId(first));
}
</script>

<template>
  <aside class="left-rail">
    <div class="section">
      <div class="section-title">Project</div>
      <div class="project-name">{{ currentProjectName }}</div>
    </div>
    <div class="section">
      <div class="section-title flex-between">
        <span>Sessions</span>
        <button class="icon" @click="newSession">+</button>
      </div>
      <ul class="list">
        <li
          v-for="s in workspace.sessions"
          :key="sessionId(s)"
          :class="['item', { active: sessionId(s) === currentId }]"
          @click="selectSession(sessionId(s))"
        >
          {{ sessionTitle(s) }}
        </li>
      </ul>
    </div>
  </aside>
</template>

<style scoped>
.left-rail {
  width: 240px;
  min-width: 240px;
  background: var(--bc-panel);
  border-right: 1px solid var(--bc-border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.section {
  padding: var(--bc-space-md);
  border-bottom: 1px solid var(--bc-border);
}
.section-title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--bc-text-dim);
  margin-bottom: var(--bc-space-sm);
}
.flex-between {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.project-name {
  font-weight: 600;
  color: var(--bc-core);
  word-break: break-all;
}
.icon {
  width: 22px;
  height: 22px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  line-height: 1;
}
.list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: var(--bc-space-xs);
}
.item {
  padding: var(--bc-space-sm) var(--bc-space-md);
  cursor: pointer;
  border-left: 2px solid transparent;
  color: var(--bc-text-muted);
  transition: background 0.12s, color 0.12s;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item:hover {
  background: var(--bc-panel-2);
  color: var(--bc-text);
}
.item.active {
  background: rgba(76, 201, 240, 0.08);
  border-left-color: var(--bc-core);
  color: var(--bc-text);
}
</style>
