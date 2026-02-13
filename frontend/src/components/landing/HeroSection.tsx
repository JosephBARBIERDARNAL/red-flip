import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faHandRock,
  faHandPaper,
  faHandScissors,
} from "@fortawesome/free-solid-svg-icons";

export default function HeroSection() {
  return (
    <section className="bg-linear-to-b from-brand-700 to-brand-800 text-white py-20 px-4">
      <div className="max-w-4xl mx-auto text-center">
        <div className="flex justify-center gap-6 text-5xl mb-8">
          <FontAwesomeIcon icon={faHandRock} className="text-brand-200" />
          <FontAwesomeIcon icon={faHandPaper} className="text-brand-300" />
          <FontAwesomeIcon icon={faHandScissors} className="text-brand-200" />
        </div>
        <p className="font-serif text-xl text-brand-200 mb-8">
          Rock. Paper. Scissors. Ranked.
        </p>
        <h1 className="font-hand text-5xl font-bold mb-4">Red Flip</h1>
        <p className="text-lg text-brand-100 mb-10 max-w-2xl mx-auto">
          Compete in real-time best-of-3 matches against other players. Climb
          the Elo leaderboard and prove your skills.
        </p>
        <div className="flex justify-center gap-4">
          <Link
            href="/play"
            className="px-6 py-3 bg-white text-brand-700 font-bold rounded-lg hover:bg-brand-50 transition-colors"
          >
            Play Now
          </Link>
          <Link
            href="/leaderboard"
            className="px-6 py-3 border-2 border-brand-300 text-brand-100 rounded-lg hover:bg-brand-600 transition-colors"
          >
            Leaderboard
          </Link>
        </div>
      </div>
    </section>
  );
}
