"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHandRock, faHandPaper, faHandScissors, IconDefinition } from "@fortawesome/free-solid-svg-icons";
import { Choice } from "@/types/game";

const choiceIcons: Record<Choice, IconDefinition> = {
  rock: faHandRock,
  paper: faHandPaper,
  scissors: faHandScissors,
};

const choiceLabels: Record<Choice, string> = {
  rock: "Rock",
  paper: "Paper",
  scissors: "Scissors",
};

interface ChoiceButtonProps {
  choice: Choice;
  selected: boolean;
  disabled: boolean;
  onClick: () => void;
}

export default function ChoiceButton({
  choice,
  selected,
  disabled,
  onClick,
}: ChoiceButtonProps) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`flex flex-col items-center gap-2 p-6 rounded-xl border-2 transition-all cursor-pointer ${
        selected
          ? "border-brand-500 bg-brand-50 ring-2 ring-brand-300"
          : disabled
          ? "border-gray-200 opacity-50 cursor-not-allowed"
          : "border-gray-200 hover:border-brand-300 hover:bg-brand-50"
      }`}
    >
      <FontAwesomeIcon
        icon={choiceIcons[choice]}
        className={`text-4xl ${selected ? "text-brand-600" : "text-gray-600"}`}
      />
      <span
        className={`font-medium ${
          selected ? "text-brand-700" : "text-gray-700"
        }`}
      >
        {choiceLabels[choice]}
      </span>
    </button>
  );
}
