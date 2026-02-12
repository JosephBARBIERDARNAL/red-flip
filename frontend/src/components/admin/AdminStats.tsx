import { PlatformStats } from "@/types/admin";
import StatsCard from "@/components/dashboard/StatsCard";
import {
  faUsers,
  faUserCheck,
  faGamepad,
  faUserSlash,
} from "@fortawesome/free-solid-svg-icons";

interface AdminStatsProps {
  stats: PlatformStats;
}

export default function AdminStats({ stats }: AdminStatsProps) {
  return (
    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
      <StatsCard icon={faUsers} label="Total Users" value={stats.total_users} />
      <StatsCard
        icon={faUserCheck}
        label="Active Users"
        value={stats.active_users}
        color="text-green-600"
      />
      <StatsCard
        icon={faGamepad}
        label="Total Matches"
        value={stats.total_matches}
      />
      <StatsCard
        icon={faUserSlash}
        label="Banned Users"
        value={stats.banned_users}
        color="text-red-600"
      />
    </div>
  );
}
