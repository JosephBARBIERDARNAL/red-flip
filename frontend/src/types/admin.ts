import { User } from "./user";

export interface PlatformStats {
  total_users: number;
  active_users: number;
  total_matches: number;
  banned_users: number;
}

export interface AdminStatsResponse {
  stats: PlatformStats;
}

export interface AdminUsersResponse {
  users: User[];
  total: number;
  page: number;
  limit: number;
}

export interface UpdateUserRequest {
  username?: string;
  elo?: number;
  wins?: number;
  losses?: number;
  draws?: number;
}

export interface BanUserRequest {
  reason: string;
}
