<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";
import { useCommandsStore, type Command } from "../../stores/commands";

const commandsStore = useCommandsStore();
const query = ref("");
const selectedIndex = ref(0);
const inputRef = ref<HTMLInputElement | null>(null);

const emit = defineEmits<{
  close: [];
}>();

const filtered = computed(() => {
  const q = query.value.toLowerCase().trim();
  if (!q) return commandsStore.commands;
  return commandsStore.commands.filter((c: Command) => {
    return c.name.toLowerCase().includes(q) || (c.description || "").toLowerCase().includes(q);
  });
});

function select(index: number) {
  if (index < 0 || index >= filtered.value.length) return;
  const cmd = filtered.value[index];
  commandsStore.executeCommand(cmd.name);
  emit("close");
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    emit("close");
    return;
  }
  if (e.key === "ArrowDown") {
    e.preventDefault();
    selectedIndex.value = Math.min(selectedIndex.value + 1, filtered.value.length - 1);
    return;
  }
  if (e.key === "ArrowUp") {
    e.preventDefault();
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
    return;
  }
  if (e.key === "Enter") {
    e.preventDefault();
    select(selectedIndex.value);
    return;
  }
  selectedIndex.value = 0;
}

watch(query, () => {
  selectedIndex.value = 0;
});

onMounted(() => {
  commandsStore.listCommands();
  inputRef.value?.focus();
  window.addEventListener("keydown", onKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown);
});
</script>

<template>
  <div class="backdrop" @click="emit('close')">
    <div class="palette" @click.stop>
      <input
        ref="inputRef"
        v-model="query"
        class="search"
        placeholder="Search commands…"
        type="text"
      />
      <div v-if="commandsStore.loading" class="empty">Loading…</div>
      <ul v-else-if="filtered.length === 0" class="list">
        <li class="empty">No matching commands</li>
      </ul>
      <ul v-else class="list">
        <li
          v-for="(cmd, i) in filtered"
          :key="cmd.name"
          :class="['item', { active: i === selectedIndex }]"
          @click="select(i)"
          @mouseenter="selectedIndex = i"
        >
          <span class="cmd-name">/{{ cmd.name }}</span>
          <span class="cmd-desc">{{ cmd.description || cmd.template || "" }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(5, 7, 11, 0.7);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 15vh;
  z-index: 200;
}
.palette {
  width: 480px;
  max-height: 360px;
  background: var(--bc-panel);
  border: 1px solid var(--bc-border);
  box-shadow: 0 16px 60px rgba(0, 0, 0, 0.6);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.search {
  width: 100%;
  padding: var(--bc-space-md);
  background: var(--bc-bg);
  border: none;
  border-bottom: 1px solid var(--bc-border);
  color: var(--bc-text);
  font-size: 14px;
  outline: none;
}
.search:focus {
  border-bottom-color: var(--bc-core);
}
.list {
  list-style: none;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  flex: 1;
}
.item {
  display: flex;
  align-items: center;
  gap: var(--bc-space-md);
  padding: var(--bc-space-sm) var(--bc-space-md);
  cursor: pointer;
  border-left: 2px solid transparent;
}
.item:hover,
.item.active {
  background: var(--bc-panel-2);
  border-left-color: var(--bc-core);
}
.cmd-name {
  font-family: var(--bc-font-mono);
  font-size: 13px;
  color: var(--bc-core);
  white-space: nowrap;
}
.cmd-desc {
  font-size: 11px;
  color: var(--bc-text-dim);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.empty {
  padding: var(--bc-space-lg);
  text-align: center;
  color: var(--bc-text-dim);
  font-size: 12px;
}
</style>
