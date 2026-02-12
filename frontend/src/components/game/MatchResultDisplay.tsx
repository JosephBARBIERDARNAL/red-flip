"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrophy, faSkull, faHandshake, faArrowUp, faArrowDown } from "@fortawesome/free-solid-svg-icons";
import { MatchResult } from "@/types/game";

interface MatchResultDisplayProps {
  result: MatchResult;
  onPlayAgain: () => void;
  onBackToMenu: () => void;
}

export default function MatchResultDisplay({
  result,
  onPlayAgain,
  onBackToMenu,
}: MatchResultDisplayProps) {
  const isWin = result.result === "win";
  const isDraw = result.result === "draw";

  return (
    <div className="max-w-lg mx-auto text-center py-8">
      <FontAwesomeIcon
        icon={isWin ? faTrophy : isDraw ? faHandshake : faSkull}
        className={`text-6xl mb-4 ${
          isWin ? "text-yellow-500" : isDraw ? "text-gray-500" : "text-red-600"
        }`}
      />

      <h2
        className={`font-serif text-4xl font-bold mb-2 ${
          isWin
            ? "text-green-600"
            : isDraw
            ? "text-gray-600"
            : "text-red-600"
        }`}
      >
        {isWin ? "Victory!" : isDraw ? "Draw" : "Defeat"}
      </h2>

      <p className="font-serif text-2xl font-bold text-brand-800 mb-4">
        {result.your_score} - {result.opponent_score}
      </p>

      {result.elo_change !== null && (
        <div className="mb-6">
          <p
            className={`text-lg font-medium ${
              result.elo_change > 0 ? "text-green-600" : result.elo_change < 0 ? "text-red-600" : "text-gray-600"
            }`}
          >
            <FontAwesomeIcon
              icon={result.elo_change >= 0 ? faArrowUp : faArrowDown}
              className="mr-1"
            />
            {result.elo_change > 0 ? "+" : ""}
            {result.elo_change} Elo
          </p>
          {result.new_elo !== null && (
            <p className="text-sm text-gray-500">New rating: {result.new_elo}</p>
          )}
        </div>
      )}

      <div className="flex justify-center gap-4">
        <button
          onClick={onPlayAgain}
          className="px-6 py-2.5 bg-brand-600 text-white font-medium rounded-lg hover:bg-brand-500 transition-colors cursor-pointer"
        >
          Play Again
        </button>
        <button
          onClick={onBackToMenu}
          className="px-6 py-2.5 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
        >
          Back to Menu
        </button>
      </div>
    </div>
  );
}
