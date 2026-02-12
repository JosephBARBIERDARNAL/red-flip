"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSpinner, faXmark } from "@fortawesome/free-solid-svg-icons";

interface MatchmakingQueueProps {
  onCancel: () => void;
}

export default function MatchmakingQueue({ onCancel }: MatchmakingQueueProps) {
  return (
    <div className="text-center py-12">
      <FontAwesomeIcon
        icon={faSpinner}
        spin
        className="text-brand-600 text-4xl mb-4"
      />
      <h2 className="font-hand text-2xl font-bold text-brand-800 mb-2">
        Finding Opponent...
      </h2>
      <p className="text-gray-600 mb-6">Waiting for another player to join</p>
      <button
        onClick={onCancel}
        className="px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
      >
        <FontAwesomeIcon icon={faXmark} className="mr-2" />
        Cancel
      </button>
    </div>
  );
}
