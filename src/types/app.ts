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
