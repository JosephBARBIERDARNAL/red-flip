"use client";

import { useState } from "react";
import { MatchRecord, MoveHistoryEntry } from "@/types/game";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faTrophy,
  faSkull,
  faHandshake,
  faChevronDown,
  faChevronUp,
} from "@fortawesome/free-solid-svg-icons";
import MoveRecapTable from "@/components/game/MoveRecapTable";

interface RecentMatchesProps {
  matches: MatchRecord[];
  userId: string;
}

interface StoredRound {
  round_number: number;
  player1_choice: string | null;
  player2_choice: string | null;
  winner: string | null;
}

function parseMoveHistory(match: MatchRecord, userId: string): MoveHistoryEntry[] {
  const isPlayer1 = match.player1_id === userId;

  try {
    const rounds = JSON.parse(match.rounds_json) as StoredRound[];
    return rounds
      .map((round) => ({
        round: round.round_number,
        playerChoice:
          (isPlayer1 ? round.player1_choice : round.player2_choice) ?? "none",
        opponentChoice:
          (isPlayer1 ? round.player2_choice : round.player1_choice) ?? "none",
        winner:
          !round.winner || round.winner === "draw"
            ? "draw"
            : round.winner === userId
              ? "you"
              : "opponent",
      }))
      .sort((a, b) => a.round - b.round);
  } catch {
    return [];
  }
}

export default function RecentMatches({ matches, userId }: RecentMatchesProps) {
  const [expandedMatchIds, setExpandedMatchIds] = useState<Set<string>>(
    () => new Set(),
  );

  const toggleExpanded = (matchId: string) => {
    setExpandedMatchIds((prev) => {
      const next = new Set(prev);
      if (next.has(matchId)) {
        next.delete(matchId);
      } else {
        next.add(matchId);
      }
      return next;
    });
  };

  if (matches.length === 0) {
    return (
      <div className="text-center py-8 text-gray-500">
        No matches played yet.
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {matches.map((match) => {
        const isPlayer1 = match.player1_id === userId;
        const myScore = isPlayer1 ? match.player1_score : match.player2_score;
        const oppScore = isPlayer1 ? match.player2_score : match.player1_score;
        const won = match.winner_id === userId;
        const isDraw = match.winner_id === null;
        const moveHistory = parseMoveHistory(match, userId);
        const isExpanded = expandedMatchIds.has(match.id);

        const myEloBefore = isPlayer1
          ? match.player1_elo_before
          : match.player2_elo_before;
        const myEloAfter = isPlayer1
          ? match.player1_elo_after
          : match.player2_elo_after;
        const eloChange =
          myEloBefore !== null && myEloAfter !== null
            ? myEloAfter - myEloBefore
            : null;

        return (
          <div
            key={match.id}
            className="p-3 rounded-lg border border-gray-100 bg-white"
          >
            <div className="flex items-center justify-between gap-4">
              <div className="flex items-center gap-3">
                <FontAwesomeIcon
                  icon={won ? faTrophy : isDraw ? faHandshake : faSkull}
                  className={
                    won
                      ? "text-yellow-500"
                      : isDraw
                        ? "text-gray-400"
                        : "text-red-500"
                  }
                />
                <div>
                  <p className="font-medium text-sm">
                    {won ? "Victory" : isDraw ? "Draw" : "Defeat"}
                  </p>
                  <p className="text-xs text-gray-400">
                    {match.finished_at
                      ? new Date(match.finished_at).toLocaleDateString()
                      : ""}
                  </p>
                </div>
              </div>
              <div className="text-right">
                <p className="font-serif font-bold text-brand-800">
                  {myScore} - {oppScore}
                </p>
                {eloChange !== null && (
                  <p
                    className={`text-xs ${
                      eloChange > 0
                        ? "text-green-600"
                        : eloChange < 0
                          ? "text-red-600"
                          : "text-gray-500"
                    }`}
                  >
                    {eloChange > 0 ? "+" : ""}
                    {eloChange}
                  </p>
                )}
              </div>
            </div>

            <div className="mt-3">
              <button
                type="button"
                onClick={() => toggleExpanded(match.id)}
                className="text-xs font-medium text-brand-700 hover:text-brand-600 cursor-pointer inline-flex items-center gap-1"
              >
                {isExpanded ? "Hide moves" : "Show moves"}
                <FontAwesomeIcon icon={isExpanded ? faChevronUp : faChevronDown} />
              </button>
            </div>

            {isExpanded && (
              <div className="mt-3">
                <MoveRecapTable
                  entries={moveHistory}
                  emptyMessage="No move recap available for this match."
                />
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
