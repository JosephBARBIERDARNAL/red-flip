import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faBolt,
  faChartLine,
  faClock,
} from "@fortawesome/free-solid-svg-icons";

const features = [
  {
    icon: faBolt,
    title: "Real-Time Matches",
    description:
      "Play best-of-3 rounds against live opponents with instant results via WebSockets.",
  },
  {
    icon: faChartLine,
    title: "Elo Ranking",
    description:
      "Every ranked match adjusts your Elo rating. Climb from 1000 to the top of the leaderboard.",
  },
  {
    icon: faClock,
    title: "15-Second Rounds",
    description:
      "Make your choice fast. If time runs out, your opponent takes the round.",
  },
];

export default function InfoSection() {
  return (
    <section className="py-16 px-4 bg-white">
      <div className="max-w-5xl mx-auto">
        <h2 className="font-hand text-3xl font-bold text-center text-brand-800 mb-12">
          How It Works
        </h2>
        <div className="grid md:grid-cols-3 gap-8">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="text-center p-6 rounded-xl border border-brand-100"
            >
              <FontAwesomeIcon
                icon={feature.icon}
                className="text-brand-600 text-3xl mb-4"
              />
              <h3 className="font-serif text-xl font-semibold text-brand-800 mb-2">
                {feature.title}
              </h3>
              <p className="text-gray-600">{feature.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
