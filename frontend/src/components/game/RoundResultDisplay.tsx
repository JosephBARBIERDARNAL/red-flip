"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faHandRock,
  faHandPaper,
  faHandScissors,
  faQuestion,
  IconDefinition,
} from "@fortawesome/free-solid-svg-icons";
import { RoundResult } from "@/types/game";

const choiceIcons: Record<string, IconDefinition> = {
  rock: faHandRock,
  paper: faHandPaper,
  scissors: faHandScissors,
};

interface RoundResultDisplayProps {
  result: RoundResult;
}

export default function RoundResultDisplay({
  result,
}: RoundResultDisplayProps) {
  const colorClass =
    result.winner === "you"
      ? "text-green-600"
      : result.winner === "opponent"
        ? "text-red-600"
        : "text-gray-600";

  const label =
    result.winner === "you"
      ? "You won the round!"
      : result.winner === "opponent"
        ? "You lost the round"
        : "Draw";

  return (
    <div className="max-w-lg mx-auto text-center py-8">
      <p className="text-sm text-gray-500 mb-2">Round {result.round}</p>
      <h2 className={`font-serif text-3xl font-bold mb-6 ${colorClass}`}>
        {label}
      </h2>

      <div className="flex justify-center items-center gap-8 mb-6">
        <div className="text-center">
          <p className="text-sm text-gray-500 mb-2">You</p>
          <FontAwesomeIcon
            icon={choiceIcons[result.your_choice] || faQuestion}
            className="text-5xl text-brand-600"
          />
          <p className="mt-2 font-medium capitalize">{result.your_choice}</p>
        </div>
        <span className="font-serif text-2xl text-gray-400">vs</span>
        <div className="text-center">
          <p className="text-sm text-gray-500 mb-2">Opponent</p>
          <FontAwesomeIcon
            icon={choiceIcons[result.opponent_choice] || faQuestion}
            className="text-5xl text-gray-600"
          />
          <p className="mt-2 font-medium capitalize">
            {result.opponent_choice}
          </p>
        </div>
      </div>

      <p className="font-serif text-xl font-bold text-brand-800">
        {result.your_score} - {result.opponent_score}
      </p>
    </div>
  );
}
