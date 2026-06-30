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

async function newSession() {
  const created = await workspace.createSession("New session");
  const id = (created as { id?: string }).id;
  if (id) await session.openSession(id);
}
</script>

<template>
  <aside class="left-rail">
    <div class="section">
      <div class="section-title">Project</div>
      <div class="project-name">{{ currentProjectName }}</div>
    </div>
    <div class="section">
      <div class="section-title">Session</div>
      <button class="new-session" @click="newSession">+ New session</button>
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
.new-session {
  width: 100%;
  font-size: 12px;
  padding: var(--bc-space-sm);
  background: var(--bc-panel-2);
}
</style>
