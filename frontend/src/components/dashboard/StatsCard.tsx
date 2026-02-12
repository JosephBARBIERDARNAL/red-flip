import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IconDefinition } from "@fortawesome/free-solid-svg-icons";

interface StatsCardProps {
  icon: IconDefinition;
  label: string;
  value: string | number;
  color?: string;
}

export default function StatsCard({
  icon,
  label,
  value,
  color = "text-brand-600",
}: StatsCardProps) {
  return (
    <div className="bg-white p-4 rounded-xl border border-gray-100 shadow-sm">
      <div className="flex items-center gap-3">
        <FontAwesomeIcon icon={icon} className={`text-xl ${color}`} />
        <div>
          <p className="text-sm text-gray-500">{label}</p>
          <p className="font-serif text-xl font-bold text-brand-800">{value}</p>
        </div>
      </div>
    </div>
  );
}
