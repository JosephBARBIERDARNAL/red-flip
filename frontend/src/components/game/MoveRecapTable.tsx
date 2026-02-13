"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faTrophy,
  faXmark,
  faEquals,
} from "@fortawesome/free-solid-svg-icons";
import { MoveHistoryEntry } from "@/types/game";

interface MoveRecapTableProps {
  entries: MoveHistoryEntry[];
  emptyMessage: string;
}

function formatChoice(choice: string) {
  return choice === "none" ? "No pick" : choice;
}

function getOutcomeStyle(winner: MoveHistoryEntry["winner"]) {
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
}

export default function MoveRecapTable({
  entries,
  emptyMessage,
}: MoveRecapTableProps) {
  return (
    <div className="rounded-lg border border-gray-200 overflow-hidden">
      <div className="grid grid-cols-[1fr_auto_1fr] gap-3 px-4 py-2 bg-gray-50 text-xs font-semibold text-gray-600">
        <p>You</p>
        <p className="text-center">Round</p>
        <p className="text-right">Opponent</p>
      </div>
      {entries.length === 0 ? (
        <p className="px-4 py-3 text-sm text-gray-500">{emptyMessage}</p>
      ) : (
        entries.map((entry) => {
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
    </div>
  );
}
