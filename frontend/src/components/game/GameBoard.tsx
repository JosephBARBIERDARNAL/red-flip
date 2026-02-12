"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faClock, faCheck } from "@fortawesome/free-solid-svg-icons";
import { Choice, OpponentInfo } from "@/types/game";
import ChoiceButton from "./ChoiceButton";

interface GameBoardProps {
  opponent: OpponentInfo;
  currentRound: number;
  timeLeft: number;
  myChoice: Choice | null;
  opponentChose: boolean;
  myScore: number;
  opponentScore: number;
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
  onChoice,
}: GameBoardProps) {
  const choices: Choice[] = ["rock", "paper", "scissors"];

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

      <p className="text-lg text-gray-700 mb-6">
        {myChoice ? "Waiting for opponent..." : "Make your choice!"}
      </p>

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
    </div>
  );
}
