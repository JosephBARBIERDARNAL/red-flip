import { User } from "./user";
import { MatchRecord } from "./game";

export interface AuthResponse {
  token: string;
  user: User;
}

export interface MeResponse {
  user: User;
}

export interface DashboardResponse {
  user: User;
  recent_matches: MatchRecord[];
  win_rate: number;
}

export interface LeaderboardResponse {
  leaderboard: User[];
}

export interface ApiError {
  error: string;
}
