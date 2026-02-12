"use client";

import { useEffect, useState } from "react";
import { api } from "@/lib/api";
import { LeaderboardResponse } from "@/types/api";
import { User } from "@/types/user";
import LeaderboardTable from "@/components/leaderboard/LeaderboardTable";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSpinner } from "@fortawesome/free-solid-svg-icons";

export default function LeaderboardPage() {
  const [players, setPlayers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api
      .get<LeaderboardResponse>("/api/leaderboard")
      .then((data) => setPlayers(data.leaderboard))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center py-20">
        <FontAwesomeIcon icon={faSpinner} spin className="text-brand-600 text-3xl" />
      </div>
    );
  }

  return (
    <div className="max-w-3xl mx-auto py-8 px-4">
      <h1 className="font-serif text-3xl font-bold text-brand-800 mb-8">
        Leaderboard
      </h1>

      {players.length === 0 ? (
        <p className="text-center text-gray-500 py-8">
          No players yet. Be the first to play!
        </p>
      ) : (
        <LeaderboardTable players={players} />
      )}
    </div>
  );
}
