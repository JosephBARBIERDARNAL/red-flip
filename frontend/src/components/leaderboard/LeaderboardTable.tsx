import { User } from "@/types/user";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMedal } from "@fortawesome/free-solid-svg-icons";

interface LeaderboardTableProps {
  players: User[];
}

const medalColors = ["text-yellow-500", "text-gray-400", "text-amber-600"];

export default function LeaderboardTable({ players }: LeaderboardTableProps) {
  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="border-b-2 border-brand-100">
            <th className="py-3 px-4 text-left text-sm font-medium text-gray-500">
              Rank
            </th>
            <th className="py-3 px-4 text-left text-sm font-medium text-gray-500">
              Player
            </th>
            <th className="py-3 px-4 text-right text-sm font-medium text-gray-500">
              Elo
            </th>
            <th className="py-3 px-4 text-right text-sm font-medium text-gray-500">
              <span className="text-green-600 font-bold">W</span>/
              <span className="text-red-600 font-bold">L</span>/D
            </th>
            <th className="py-3 px-4 text-right text-sm font-medium text-gray-500">
              Games
            </th>
          </tr>
        </thead>
        <tbody>
          {players.map((player, i) => (
            <tr
              key={player.id}
              className="border-b border-gray-50 hover:bg-brand-50 transition-colors"
            >
              <td className="py-3 px-4">
                {i < 3 ? (
                  <FontAwesomeIcon icon={faMedal} className={medalColors[i]} />
                ) : (
                  <span className="text-gray-500 text-sm">{i + 1}</span>
                )}
              </td>
              <td className="py-3 px-4 font-medium text-brand-800">
                {player.username}
              </td>
              <td className="py-3 px-4 text-right font-serif font-bold text-brand-700">
                {player.elo}
              </td>
              <td className="py-3 px-4 text-right text-sm text-gray-600">
                <span className="text-green-600 font-bold">{player.wins}</span>/
                <span className="text-red-600 font-bold">{player.losses}</span>/
                {player.draws}
              </td>
              <td className="py-3 px-4 text-right text-sm text-gray-500">
                {player.total_games}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
