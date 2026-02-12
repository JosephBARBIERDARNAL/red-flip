export interface User {
  id: string;
  username: string;
  avatar_url: string | null;
  elo: number;
  total_games: number;
  wins: number;
  losses: number;
  draws: number;
}
