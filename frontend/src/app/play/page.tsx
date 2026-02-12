"use client";

import { useRouter } from "next/navigation";
import { useAuth } from "@/hooks/useAuth";
import { useWebSocket } from "@/hooks/useWebSocket";
import { useGameState } from "@/hooks/useGameState";
import ModeSelector from "@/components/game/ModeSelector";
import MatchmakingQueue from "@/components/game/MatchmakingQueue";
import GameBoard from "@/components/game/GameBoard";
import RoundResultDisplay from "@/components/game/RoundResultDisplay";
import MatchResultDisplay from "@/components/game/MatchResultDisplay";
import GuestBanner from "@/components/game/GuestBanner";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSpinner,
  faExclamationTriangle,
} from "@fortawesome/free-solid-svg-icons";

export default function PlayPage() {
  const router = useRouter();
  const { user, token, loading } = useAuth();
  const isGuest = !user && !loading;
  // Only allow guest connection when auth loading is complete
  const { connected, send, addMessageHandler } = useWebSocket(
    token,
    isGuest
  );
  const game = useGameState({ send, addMessageHandler });

  if (loading) {
    return (
      <div className="flex justify-center py-20">
        <FontAwesomeIcon
          icon={faSpinner}
          spin
          className="text-brand-600 text-3xl"
        />
      </div>
    );
  }

  if (!connected) {
    return (
      <div className="text-center py-20">
        <FontAwesomeIcon
          icon={faSpinner}
          spin
          className="text-brand-600 text-3xl mb-4"
        />
        <p className="text-gray-600">Connecting to game server...</p>
      </div>
    );
  }

  if (game.error) {
    return (
      <div className="max-w-lg mx-auto text-center py-20 px-4">
        <FontAwesomeIcon
          icon={faExclamationTriangle}
          className="text-yellow-500 text-3xl mb-4"
        />
        <p className="text-gray-800 font-medium mb-4">{game.error}</p>
        <button
          onClick={game.resetGame}
          className="px-4 py-2 bg-brand-600 text-white rounded-lg hover:bg-brand-500 transition-colors cursor-pointer"
        >
          Back to Menu
        </button>
      </div>
    );
  }

  return (
    <div className="py-8 px-4">
      {isGuest && <GuestBanner />}

      {game.status === "idle" && (
        <ModeSelector
          onSelect={(ranked) => game.joinQueue(ranked)}
          isGuest={isGuest}
        />
      )}

      {game.status === "queued" && (
        <MatchmakingQueue onCancel={game.leaveQueue} />
      )}

      {game.status === "playing" && game.opponent && (
        <GameBoard
          opponent={game.opponent}
          currentRound={game.currentRound}
          timeLeft={game.timeLeft}
          myChoice={game.myChoice}
          opponentChose={game.opponentChose}
          myScore={game.roundResult?.your_score ?? 0}
          opponentScore={game.roundResult?.opponent_score ?? 0}
          onChoice={game.makeChoice}
        />
      )}

      {game.status === "round_result" && game.roundResult && (
        <RoundResultDisplay result={game.roundResult} />
      )}

      {game.status === "match_complete" && game.matchResult && (
        <MatchResultDisplay
          result={game.matchResult}
          onPlayAgain={game.resetGame}
          onBackToMenu={() => router.push("/dashboard")}
        />
      )}
    </div>
  );
}
