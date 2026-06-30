<script setup lang="ts">
const props = defineProps<{
  values: number[];
  labels: string[];
}>();

const size = 160;
const center = size / 2;
const radius = 60;
const axes = props.labels.length || 1;

function point(value: number, index: number) {
  const angle = (Math.PI * 2 * index) / axes - Math.PI / 2;
  const r = (Math.min(Math.max(value, 0), 100) / 100) * radius;
  return {
    x: center + r * Math.cos(angle),
    y: center + r * Math.sin(angle),
  };
}

function gridPoint(level: number, index: number) {
  const angle = (Math.PI * 2 * index) / axes - Math.PI / 2;
  const r = (level / 5) * radius;
  return {
    x: center + r * Math.cos(angle),
    y: center + r * Math.sin(angle),
  };
}

const polygonPoints = props.values
  .map((v, i) => {
    const p = point(v, i);
    return `${p.x},${p.y}`;
  })
  .join(" ");
</script>

<template>
  <svg class="radar" :viewBox="`0 0 ${size} ${size}`">
    <g v-for="level in 5" :key="level">
      <polygon
        :points="
          labels
            .map((_, i) => {
              const p = gridPoint(level, i);
              return `${p.x},${p.y}`;
            })
            .join(' ')
        "
        fill="none"
        stroke="var(--bc-border)"
        stroke-width="1"
      />
    </g>
    <line
      v-for="(_, i) in labels"
      :key="`axis-${i}`"
      :x1="center"
      :y1="center"
      :x2="gridPoint(5, i).x"
      :y2="gridPoint(5, i).y"
      stroke="var(--bc-border)"
      stroke-width="1"
    />
    <polygon
      :points="polygonPoints"
      fill="rgba(255, 90, 54, 0.18)"
      stroke="var(--bc-flame)"
      stroke-width="2"
    />
    <circle
      v-for="(v, i) in values"
      :key="`dot-${i}`"
      :cx="point(v, i).x"
      :cy="point(v, i).y"
      r="3"
      fill="var(--bc-flame)"
    />
    <text
      v-for="(label, i) in labels"
      :key="`label-${i}`"
      :x="gridPoint(6, i).x"
      :y="gridPoint(6, i).y"
      text-anchor="middle"
      dominant-baseline="middle"
      fill="var(--bc-text-dim)"
      font-size="9"
    >
      {{ label }}
    </text>
  </svg>
</template>

<style scoped>
.radar {
  width: 100%;
  height: auto;
}
</style>
