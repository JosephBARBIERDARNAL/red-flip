"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faClock,
  faCheck,
  faTrophy,
  faXmark,
  faEquals,
} from "@fortawesome/free-solid-svg-icons";
import { Choice, OpponentInfo, MoveHistoryEntry } from "@/types/game";
import ChoiceButton from "./ChoiceButton";

interface GameBoardProps {
  opponent: OpponentInfo;
  currentRound: number;
  timeLeft: number;
  myChoice: Choice | null;
  opponentChose: boolean;
  myScore: number;
  opponentScore: number;
  moveHistory: MoveHistoryEntry[];
  onChoice: (choice: Choice) => void;
}

export default function GameBoard({
  opponent,
  currentRound,
  timeLeft,
  myChoice,
  opponentChose,
  myScore,
  opponentScore,
  moveHistory,
  onChoice,
}: GameBoardProps) {
  const choices: Choice[] = ["rock", "paper", "scissors"];
  const formatChoice = (choice: string) =>
    choice === "none" ? "No pick" : choice;

  const getOutcomeStyle = (winner: MoveHistoryEntry["winner"]) => {
    if (winner === "you") {
      return {
        row: "bg-green-50",
        badge: "bg-green-100 text-green-700",
        icon: faTrophy,
        label: "Won",
      };
    }
    if (winner === "opponent") {
      return {
        row: "bg-red-50",
        badge: "bg-red-100 text-red-700",
        icon: faXmark,
        label: "Lost",
      };
    }
    return {
      row: "bg-gray-50",
      badge: "bg-gray-200 text-gray-700",
      icon: faEquals,
      label: "Draw",
    };
  };

  return (
    <div className="max-w-lg mx-auto text-center">
      <div className="flex justify-between items-center mb-6">
        <div className="text-left">
          <p className="text-sm text-gray-500">Opponent</p>
          <p className="font-serif font-semibold text-brand-800">
            {opponent.username}
          </p>
          <p className="text-xs text-gray-400">Elo: {opponent.elo}</p>
        </div>
        <div className="text-center">
          <p className="text-sm text-gray-500">Round {currentRound}</p>
          <p className="font-serif text-2xl font-bold text-brand-800">
            {myScore} - {opponentScore}
          </p>
        </div>
        <div className="text-right">
          <div
            className={`flex items-center gap-1 text-sm ${
              timeLeft <= 5 ? "text-red-600 font-bold" : "text-gray-500"
            }`}
          >
            <FontAwesomeIcon icon={faClock} />
            {timeLeft}s
          </div>
          {opponentChose && (
            <p className="text-xs text-green-600 mt-1">
              <FontAwesomeIcon icon={faCheck} className="mr-1" />
              Opponent chose
            </p>
          )}
        </div>
      </div>

      <p className="text-lg text-gray-700 mb-2">
        {myChoice ? "Waiting for opponent..." : "Make your choice!"}
      </p>

      <p className="text-sm text-black mb-4">First to win 3 rounds</p>

      <div className="flex justify-center gap-4">
        {choices.map((choice) => (
          <ChoiceButton
            key={choice}
            choice={choice}
            selected={myChoice === choice}
            disabled={myChoice !== null}
            onClick={() => onChoice(choice)}
          />
        ))}
      </div>

      <div className="mt-8 text-left">
        <p className="text-sm font-semibold text-gray-700 mb-2">Past moves</p>
        <div className="rounded-lg border border-gray-200 overflow-hidden">
          <div className="grid grid-cols-[1fr_auto_1fr] gap-3 px-4 py-2 bg-gray-50 text-xs font-semibold text-gray-600">
            <p>You</p>
            <p className="text-center">Round</p>
            <p className="text-right">Opponent</p>
          </div>
          {moveHistory.length === 0 ? (
            <p className="px-4 py-3 text-sm text-gray-500">
              No moves yet this match.
            </p>
          ) : (
            moveHistory.map((entry) => {
              const outcome = getOutcomeStyle(entry.winner);
              return (
                <div
                  key={entry.round}
                  className={`grid grid-cols-[1fr_auto_1fr] gap-3 px-4 py-2 border-t border-gray-100 text-sm ${outcome.row}`}
                >
                  <div className="flex items-center gap-2">
                    <span className="inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-semibold bg-brand-100 text-brand-700">
                      You
                    </span>
                    <p className="capitalize font-medium">
                      {formatChoice(entry.playerChoice)}
                    </p>
                  </div>
                  <div className="text-center">
                    <p className="text-xs text-gray-500">{entry.round}</p>
                    <span
                      className={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[11px] font-semibold ${outcome.badge}`}
                    >
                      <FontAwesomeIcon icon={outcome.icon} />
                      {outcome.label}
                    </span>
                  </div>
                  <div className="flex items-center justify-end gap-2">
                    <p className="text-right capitalize font-medium">
                      {formatChoice(entry.opponentChoice)}
                    </p>
                    <span className="inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-semibold bg-gray-200 text-gray-700">
                      Opp
                    </span>
                  </div>
                </div>
              );
            })
          )}
          <div className="grid grid-cols-[1fr_auto_1fr] gap-3 px-4 py-2 border-t border-gray-200 bg-gray-50 text-sm font-semibold text-gray-700">
            <p>Score: {myScore}</p>
            <p className="text-center">Now</p>
            <p className="text-right">Score: {opponentScore}</p>
          </div>
        </div>
      </div>
    </div>
  );
}
