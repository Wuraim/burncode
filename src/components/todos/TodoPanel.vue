<script setup lang="ts">
import { useSessionStore } from "../../stores/session";

const session = useSessionStore();

function todoText(t: unknown) {
  return (t as { content?: string }).content || "Todo";
}
function todoDone(t: unknown) {
  return (t as { status?: string }).status === "completed";
}
</script>

<template>
  <div class="todo-panel">
    <div class="title">Todos</div>
    <ul class="list">
      <li v-for="(t, i) in session.todos" :key="i" class="row" :class="{ done: todoDone(t) }">
        <span class="check">{{ todoDone(t) ? "✓" : "○" }}</span>
        <span class="text">{{ todoText(t) }}</span>
      </li>
      <li v-if="session.todos.length === 0" class="empty">No active todos</li>
    </ul>
  </div>
</template>

<style scoped>
.todo-panel {
  padding: var(--bc-space-md);
  flex: 1;
  overflow: auto;
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
  gap: var(--bc-space-sm);
  font-size: 12px;
  color: var(--bc-text-muted);
}
.row.done .text {
  text-decoration: line-through;
  color: var(--bc-text-dim);
}
.check {
  color: var(--bc-core);
}
.empty {
  color: var(--bc-text-dim);
  font-size: 12px;
}
</style>
