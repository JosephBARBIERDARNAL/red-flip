import { MatchRecord } from "@/types/game";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrophy, faSkull, faHandshake } from "@fortawesome/free-solid-svg-icons";

interface RecentMatchesProps {
  matches: MatchRecord[];
  userId: string;
}

export default function RecentMatches({ matches, userId }: RecentMatchesProps) {
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
            className="flex items-center justify-between p-3 rounded-lg border border-gray-100 bg-white"
          >
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
                    eloChange > 0 ? "text-green-600" : eloChange < 0 ? "text-red-600" : "text-gray-500"
                  }`}
                >
                  {eloChange > 0 ? "+" : ""}
                  {eloChange}
                </p>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
