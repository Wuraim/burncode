import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invokeCommand } from "../services/tauri";
import type {
  ModelUsageStats,
  ProviderInfo,
  ProviderQuotaConfig,
  ProviderAuthMethod,
  ProviderModelInfo,
  RouteConfig,
} from "../types/app";

export const useProvidersStore = defineStore("providers", () => {
  const all = ref<ProviderInfo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const PINNED = ["openai", "opencode"];

  let pollTimer: ReturnType<typeof setInterval> | null = null;

  const pinnedProviders = computed(() => all.value.filter((p) => PINNED.includes(p.id)));
  const otherProviders = computed(() => all.value.filter((p) => !PINNED.includes(p.id)));

  async function refresh() {
    loading.value = true;
    error.value = null;
    try {
      const [providersData, routeConfig, authMethodsData] = await Promise.all([
        invokeCommand<{ all?: unknown[]; connected?: string[]; default?: Record<string, string> }>("list_providers"),
        invokeCommand<RouteConfig>("get_route_config"),
        invokeCommand<Record<string, ProviderAuthMethod[]>>("list_provider_auth_methods"),
      ]);

      const rawAll = (providersData?.all ?? []) as { id?: string; name?: string; models?: Record<string, unknown> }[];
      const connected = new Set(providersData?.connected ?? []);
      const defaults = providersData?.default ?? {};
      const quotas = (routeConfig?.analytics?.provider_quotas ?? {}) as Record<string, ProviderQuotaConfig>;
      const authMap = authMethodsData ?? {};

      const allModels = (routeConfig?.analytics?.models ?? {}) as Record<string, ModelUsageStats>;

      all.value = rawAll.map((raw) => {
        const id = raw.id || "unknown";
        const rawModels = raw.models ?? {};
        const models: ProviderModelInfo[] = Object.entries(rawModels).map(([mid, m]) => {
          const modelRaw = m as {
            id?: string; name?: string; status?: string;
            capabilities?: Record<string, unknown>;
            cost?: Record<string, unknown>;
            limit?: Record<string, unknown>;
          };
          const modelId = modelRaw.id || mid;
          return {
            id: modelId,
            name: modelRaw.name || mid,
            status: modelRaw.status || "active",
            capabilities: (modelRaw.capabilities ?? {}) as unknown as ProviderModelInfo["capabilities"],
            cost: (modelRaw.cost ?? {}) as unknown as ProviderModelInfo["cost"],
            limit: (modelRaw.limit ?? {}) as unknown as ProviderModelInfo["limit"],
            usage: allModels[id + "/" + modelId] ?? null,
          };
        });

        // Compute aggregate usage from all models for this provider
        const agg: ModelUsageStats = {
          requests: 0, successes: 0, failures: 0,
          total_cost: 0, total_input_tokens: 0, total_output_tokens: 0,
          total_reasoning_tokens: 0, total_latency_ms: 0,
          by_difficulty: {},
        };
        for (const m of models) {
          if (!m.usage) continue;
          agg.requests += m.usage.requests;
          agg.successes += m.usage.successes;
          agg.failures += m.usage.failures;
          agg.total_cost += m.usage.total_cost;
          agg.total_input_tokens += m.usage.total_input_tokens;
          agg.total_output_tokens += m.usage.total_output_tokens;
          agg.total_reasoning_tokens += m.usage.total_reasoning_tokens;
          agg.total_latency_ms += m.usage.total_latency_ms;
          for (const [d, c] of Object.entries(m.usage.by_difficulty)) {
            agg.by_difficulty[d] = (agg.by_difficulty[d] || 0) + c;
          }
        }

        return {
          id,
          name: raw.name || id,
          connected: connected.has(id),
          defaultModel: (defaults as Record<string, string>)[id] || "",
          models,
          quota: quotas[id] ?? {
            source: "estimated" as const,
            used_requests: 0,
            used_budget: 0,
            used_tokens: 0,
          },
          authMethods: authMap[id] ?? [],
          aggregateUsage: agg,
        };
      });
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function setQuota(
    providerId: string,
    requestsCap?: number,
    budgetCap?: number,
    tokensCap?: number,
    periodType?: "daily" | "monthly"
  ) {
    await invokeCommand("set_provider_quota", {
      providerId,
      requestsCap: requestsCap ?? null,
      budgetCap: budgetCap ?? null,
      tokensCap: tokensCap ?? null,
      periodType: periodType ?? null,
    });
    await refresh();
  }

  async function connectProvider(providerId: string, apiKey: string) {
    await invokeCommand("set_provider_auth", {
      providerId,
      body: { type: "api", key: apiKey },
    });
    await refresh();
  }

  function startPolling(intervalMs = 60000) {
    stopPolling();
    pollTimer = setInterval(() => { refresh(); }, intervalMs);
  }

  function stopPolling() {
    if (pollTimer !== null) {
      clearInterval(pollTimer);
      pollTimer = null;
    }
  }

  return {
    all,
    pinnedProviders,
    otherProviders,
    loading,
    error,
    refresh,
    setQuota,
    connectProvider,
    startPolling,
    stopPolling,
  };
});
