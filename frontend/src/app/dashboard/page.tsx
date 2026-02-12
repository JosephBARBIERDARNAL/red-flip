"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/hooks/useAuth";
import { api } from "@/lib/api";
import { DashboardResponse } from "@/types/api";
import StatsCard from "@/components/dashboard/StatsCard";
import RecentMatches from "@/components/dashboard/RecentMatches";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSpinner,
  faChartLine,
  faGamepad,
  faTrophy,
  faSkull,
  faPercent,
} from "@fortawesome/free-solid-svg-icons";

export default function DashboardPage() {
  const { user, loading: authLoading } = useAuth();
  const router = useRouter();
  const [data, setData] = useState<DashboardResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
      return;
    }

    if (user) {
      api
        .get<DashboardResponse>("/api/dashboard")
        .then(setData)
        .catch(() => {})
        .finally(() => setLoading(false));
    }
  }, [user, authLoading, router]);

  if (authLoading || loading) {
    return (
      <div className="flex justify-center py-20">
        <FontAwesomeIcon icon={faSpinner} spin className="text-brand-600 text-3xl" />
      </div>
    );
  }

  if (!data) return null;

  return (
    <div className="max-w-4xl mx-auto py-8 px-4">
      <h1 className="font-serif text-3xl font-bold text-brand-800 mb-8">
        Dashboard
      </h1>

      <div className="grid grid-cols-2 md:grid-cols-5 gap-4 mb-8">
        <StatsCard icon={faChartLine} label="Elo" value={data.user.elo} />
        <StatsCard
          icon={faGamepad}
          label="Games"
          value={data.user.total_games}
        />
        <StatsCard
          icon={faTrophy}
          label="Wins"
          value={data.user.wins}
          color="text-green-600"
        />
        <StatsCard
          icon={faSkull}
          label="Losses"
          value={data.user.losses}
          color="text-red-600"
        />
        <StatsCard
          icon={faPercent}
          label="Win Rate"
          value={`${data.win_rate}%`}
        />
      </div>

      <h2 className="font-serif text-xl font-bold text-brand-800 mb-4">
        Recent Matches
      </h2>
      <RecentMatches matches={data.recent_matches} userId={data.user.id} />
    </div>
  );
}
