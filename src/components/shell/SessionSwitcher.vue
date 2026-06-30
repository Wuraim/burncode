<script setup lang="ts">
import { ref, computed } from "vue";
import { invokeCommand } from "../../services/tauri";
import { useWorkspaceStore } from "../../stores/workspace";
import { useSessionStore } from "../../stores/session";

const workspace = useWorkspaceStore();
const session = useSessionStore();
const open = ref(false);
const editing = ref<string | null>(null);
const renameText = ref("");

function sessionTitle(s: unknown) {
  const item = s as { id?: string; title?: string; directory?: string };
  return item.title || item.directory || item.id || "Session";
}

function sessionId(s: unknown) {
  return (s as { id?: string }).id || "";
}

const currentTitle = computed(() => {
  if (!session.currentSession) return "No session";
  return sessionTitle(session.currentSession);
});

async function select(id: string) {
  open.value = false;
  await session.openSession(id);
}

async function create() {
  open.value = false;
  const created = await workspace.createSession("New session");
  const id = (created as { id?: string }).id;
  if (id) await session.openSession(id);
}

function startRename(id: string, current: string) {
  editing.value = id;
  renameText.value = current;
}

async function confirmRename(id: string) {
  if (!renameText.value.trim()) {
    editing.value = null;
    return;
  }
  await invokeCommand("update_session", { id, title: renameText.value });
  editing.value = null;
  // Reload sessions
  await workspace.loadSessions();
  if (session.currentSession && sessionId(session.currentSession) === id) {
    await session.openSession(id);
  }
}

async function confirmDelete(id: string) {
  await invokeCommand("delete_session", { id });
  editing.value = null;
  open.value = false;
  await workspace.loadSessions();
  // If the deleted session was active, switch to another
  if (session.currentSession && sessionId(session.currentSession) === id) {
    const next = workspace.sessions[0] as { id?: string } | undefined;
    if (next?.id) await session.openSession(next.id);
  }
}
</script>

<template>
  <div class="switcher">
    <button class="trigger" @click="open = !open">
      <span class="label">Session</span>
      <span class="title">{{ currentTitle }}</span>
      <span class="caret">▼</span>
    </button>
    <div v-if="open" class="dropdown">
      <div class="header">
        <span>Past sessions</span>
        <button class="small" @click="create">+ New</button>
      </div>
      <ul class="list">
        <li
          v-for="s in workspace.sessions"
          :key="sessionId(s)"
          :class="['item', { active: sessionId(s) === sessionId(session.currentSession) }]"
        >
          <template v-if="editing === sessionId(s)">
            <input
              v-model="renameText"
              class="rename-input"
              @keydown.enter.prevent="confirmRename(sessionId(s))"
              @keydown.escape.prevent="editing = null"
              @blur="confirmRename(sessionId(s))"
              autofocus
            />
          </template>
          <template v-else>
            <span class="item-title" @click="select(sessionId(s))">{{ sessionTitle(s) }}</span>
            <span class="item-actions">
              <button class="icon-btn" title="Rename" @click.stop="startRename(sessionId(s), sessionTitle(s))">✎</button>
              <button class="icon-btn danger" title="Delete" @click.stop="confirmDelete(sessionId(s))">✕</button>
            </span>
          </template>
        </li>
        <li v-if="workspace.sessions.length === 0" class="empty">No sessions</li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.switcher {
  position: relative;
}
.trigger {
  display: flex;
  align-items: center;
  gap: var(--bc-space-sm);
  background: var(--bc-panel-2);
  border: 1px solid var(--bc-border);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 12px;
}
.label {
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 10px;
}
.title {
  color: var(--bc-text);
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.caret {
  color: var(--bc-text-dim);
  font-size: 8px;
}
.dropdown {
  position: absolute;
  top: calc(100% + var(--bc-space-xs));
  left: 0;
  min-width: 300px;
  background: var(--bc-panel);
  border: 1px solid var(--bc-border);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
  z-index: 100;
}
.dropdown .header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--bc-space-sm) var(--bc-space-md);
  border-bottom: 1px solid var(--bc-border);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--bc-text-dim);
}
.dropdown .list {
  list-style: none;
  margin: 0;
  padding: var(--bc-space-sm) 0;
  max-height: 300px;
  overflow-y: auto;
}
.item {
  display: flex;
  align-items: center;
  padding: var(--bc-space-sm) var(--bc-space-md);
  cursor: pointer;
  color: var(--bc-text-muted);
  font-size: 12px;
}
.item:hover {
  background: var(--bc-panel-2);
}
.item.active {
  background: rgba(76, 201, 240, 0.08);
  color: var(--bc-core);
}
.item-title {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item-actions {
  display: none;
  gap: var(--bc-space-xs);
  margin-left: var(--bc-space-sm);
}
.item:hover .item-actions {
  display: flex;
}
.icon-btn {
  background: none;
  border: none;
  color: var(--bc-text-dim);
  font-size: 11px;
  padding: var(--bc-space-xs);
  cursor: pointer;
  line-height: 1;
}
.icon-btn.danger:hover {
  color: var(--bc-error);
}
.rename-input {
  width: 100%;
  background: var(--bc-bg);
  border: 1px solid var(--bc-core);
  color: var(--bc-text);
  padding: var(--bc-space-xs) var(--bc-space-sm);
  font-size: 12px;
  outline: none;
}
.empty {
  padding: var(--bc-space-md);
  color: var(--bc-text-dim);
  font-size: 12px;
}
button.small {
  font-size: 11px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
}
</style>
