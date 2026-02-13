export type Choice = "rock" | "paper" | "scissors";

export type GameResult = "win" | "loss" | "draw";

export type GameStatus =
  | "idle"
  | "queued"
  | "playing"
  | "round_result"
  | "match_complete";

export interface OpponentInfo {
  username: string;
  elo: number;
}

export interface RoundResult {
  round: number;
  your_choice: string;
  opponent_choice: string;
  winner: RoundWinner;
  your_score: number;
  opponent_score: number;
}

export interface MatchResult {
  result: GameResult;
  your_score: number;
  opponent_score: number;
  elo_change: number | null;
  new_elo: number | null;
}

export type RoundWinner = "you" | "opponent" | "draw";

export interface MoveHistoryEntry {
  round: number;
  playerChoice: string;
  opponentChoice: string;
  winner: RoundWinner;
}

export interface MatchRecord {
  id: string;
  player1_id: string;
  player2_id: string;
  winner_id: string | null;
  is_ranked: boolean;
  player1_score: number;
  player2_score: number;
  rounds_json: string;
  player1_elo_before: number | null;
  player1_elo_after: number | null;
  player2_elo_before: number | null;
  player2_elo_after: number | null;
  status: string;
  created_at: string;
  finished_at: string | null;
}
