export interface ConnectionProfile {
  server_url: string;
  username: string;
  password: string;
  auto_connect: boolean;
}

export interface AppSettings {
  profile?: ConnectionProfile;
  routing?: RoutingSettings;
}

export interface RoutingSettings {
  default_provider?: string;
  budget_tokens_per_session?: number;
}

export interface RouteBucketMeta {
  name: string;
  description: string;
}

export interface RoutePolicyMeta {
  buckets: RouteBucketMeta[];
  selection_rule: string;
}

export interface RouteCandidate {
  provider_id: string;
  model_id: string;
  score: number;
  reason: string;
}

export interface RouteDecision {
  provider_id: string;
  model_id: string;
  difficulty: string;
  reason: string;
  source: string;
  candidates: RouteCandidate[];
}

export interface ModelUsageStats {
  requests: number;
  successes: number;
  failures: number;
  total_cost: number;
  total_input_tokens: number;
  total_output_tokens: number;
  total_reasoning_tokens: number;
  total_latency_ms: number;
  by_difficulty: Record<string, number>;
  last_used_at_ms?: number;
}

export interface ProviderQuotaConfig {
  requests_cap?: number;
  budget_cap?: number;
  tokens_cap?: number;
  source: "estimated" | "manual" | "live";
  used_requests: number;
  used_budget: number;
  used_tokens: number;
  period_type?: "daily" | "monthly";
  period_start_ms?: number;
  period_cap_requests?: number;
  period_cap_budget?: number;
  period_cap_tokens?: number;
}

export interface ModelCapabilities {
  toolcall: boolean;
  reasoning: boolean;
  attachment: boolean;
  input: { text: boolean; audio: boolean; image: boolean; video: boolean; pdf: boolean };
  output: { text: boolean; audio: boolean; image: boolean; video: boolean; pdf: boolean };
}

export interface ModelCost {
  input: number;
  output: number;
  cache: { read: number; write: number };
}

export interface ModelLimit {
  context: number;
  output: number;
}

export interface ModelInfo {
  id: string;
  name: string;
  status: string;
  capabilities: ModelCapabilities;
  cost: ModelCost;
  limit: ModelLimit;
}

export interface ProviderModelInfo extends ModelInfo {
  usage: ModelUsageStats | null;
}

export interface ProviderInfo {
  id: string;
  name: string;
  connected: boolean;
  defaultModel: string;
  models: ProviderModelInfo[];
  quota: ProviderQuotaConfig;
  authMethods: ProviderAuthMethod[];
  aggregateUsage: ModelUsageStats;
}

export interface RoutingAnalytics {
  models: Record<string, ModelUsageStats>;
  provider_quotas: Record<string, ProviderQuotaConfig>;
  total_routes: number;
  total_failures: number;
}

export interface RouteConfig {
  settings: RoutingSettings;
  policy: RoutePolicyMeta;
  providers: unknown;
  defaults: unknown;
  analytics: RoutingAnalytics;
  pinned_providers: string[];
}

export interface OpencodeEvent {
  event_type: string;
  data: unknown;
}

export interface Session {
  id: string;
  title?: string;
  projectID?: string;
  [key: string]: unknown;
}

export interface Project {
  id: string;
  name?: string;
  path?: string;
  [key: string]: unknown;
}

export interface Provider {
  id: string;
  name?: string;
  connected?: boolean;
  [key: string]: unknown;
}

export interface ProviderAuthMethod {
  type: string;
  label: string;
}

export interface Agent {
  name: string;
  description?: string;
  mode?: string;
  builtIn?: boolean;
  [key: string]: unknown;
}
