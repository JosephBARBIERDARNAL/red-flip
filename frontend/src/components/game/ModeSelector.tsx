"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrophy, faGamepad } from "@fortawesome/free-solid-svg-icons";

interface ModeSelectorProps {
  onSelect: (ranked: boolean) => void;
  isGuest?: boolean;
}

export default function ModeSelector({
  onSelect,
  isGuest = false,
}: ModeSelectorProps) {
  return (
    <div className="text-center py-8">
      <h2 className="font-hand text-3xl font-bold text-brand-800 mb-8">
        Choose Game Mode
      </h2>
      <div className="flex justify-center gap-6">
        <button
          onClick={() => !isGuest && onSelect(true)}
          disabled={isGuest}
          className={`w-52 p-6 rounded-xl border-2 transition-all ${
            isGuest
              ? "border-gray-200 bg-gray-100 opacity-60 cursor-not-allowed"
              : "border-brand-200 hover:border-brand-500 hover:bg-brand-50 cursor-pointer"
          }`}
        >
          <FontAwesomeIcon
            icon={faTrophy}
            className={`text-3xl mb-3 ${isGuest ? "text-gray-400" : "text-brand-600"}`}
          />
          <h3
            className={`font-serif text-xl font-semibold mb-1 ${isGuest ? "text-gray-500" : "text-brand-800"}`}
          >
            Ranked
          </h3>
          <p className="text-sm text-gray-600">
            {isGuest ? "Requires account" : "Affects your Elo rating"}
          </p>
        </button>
        <button
          onClick={() => onSelect(false)}
          className="w-52 p-6 rounded-xl border-2 border-gray-200 hover:border-gray-400 hover:bg-gray-50 transition-all cursor-pointer"
        >
          <FontAwesomeIcon
            icon={faGamepad}
            className="text-gray-500 text-3xl mb-3"
          />
          <h3 className="font-serif text-xl font-semibold text-gray-800 mb-1">
            Casual
          </h3>
          <p className="text-sm text-gray-600">Just for fun</p>
        </button>
      </div>
    </div>
  );
}
