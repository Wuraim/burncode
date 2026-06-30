<script setup lang="ts">
import { onMounted } from "vue";
import LeftRail from "../components/shell/LeftRail.vue";
import RightRail from "../components/shell/RightRail.vue";
import SessionStream from "../components/session/SessionStream.vue";
import PromptComposer from "../components/session/PromptComposer.vue";
import ActivityTimeline from "../components/session/ActivityTimeline.vue";
import DiffPanel from "../components/diff/DiffPanel.vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useConnectionStore } from "../stores/connection";

const workspace = useWorkspaceStore();
const connection = useConnectionStore();

onMounted(() => {
  workspace.loadAll();
});
</script>

<template>
  <div class="workspace">
    <div class="topbar">
      <div class="brand">BURNCODE</div>
      <div class="status">
        <span v-if="connection.serverVersion" class="version">
          opencode {{ connection.serverVersion }}
        </span>
        <button class="disconnect" @click="connection.disconnect">Disconnect</button>
      </div>
    </div>
    <div class="body">
      <LeftRail />
      <main class="center">
        <SessionStream />
        <ActivityTimeline />
        <DiffPanel />
        <PromptComposer />
      </main>
      <RightRail />
    </div>
  </div>
</template>

<style scoped>
.workspace {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bc-bg);
}
.topbar {
  height: 42px;
  min-height: 42px;
  background: var(--bc-panel);
  border-bottom: 1px solid var(--bc-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--bc-space-md);
}
.brand {
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0.16em;
  background: linear-gradient(90deg, var(--bc-core), var(--bc-violet));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}
.status {
  display: flex;
  align-items: center;
  gap: var(--bc-space-md);
}
.version {
  font-size: 11px;
  color: var(--bc-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.disconnect {
  font-size: 11px;
  padding: var(--bc-space-xs) var(--bc-space-sm);
  background: transparent;
}
.body {
  flex: 1;
  display: flex;
  overflow: hidden;
}
.center {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
