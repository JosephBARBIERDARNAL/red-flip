"use client";

import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faUserPlus, faTimes } from "@fortawesome/free-solid-svg-icons";
import { useState } from "react";

export default function GuestBanner() {
  const [dismissed, setDismissed] = useState(false);

  if (dismissed) return null;

  return (
    <div className="bg-gradient-to-r from-brand-600 to-brand-500 text-white px-4 py-3 rounded-lg shadow-lg mb-6 relative">
      <button
        onClick={() => setDismissed(true)}
        className="absolute top-2 right-2 text-white/80 hover:text-white transition-colors"
        aria-label="Dismiss"
      >
        <FontAwesomeIcon icon={faTimes} className="w-4 h-4" />
      </button>
      <div className="flex flex-col sm:flex-row items-start sm:items-center gap-3">
        <div className="flex-1">
          <h3 className="font-semibold text-lg mb-1">
            Playing as Guest - Unranked Mode
          </h3>
          <p className="text-white/90 text-sm">
            Create an account to improve your Elo, play ranked matches, and
            track your match history!
          </p>
        </div>
        <Link
          href="/register"
          className="flex items-center gap-2 bg-white text-brand-600 px-4 py-2 rounded-lg font-medium hover:bg-gray-100 transition-colors whitespace-nowrap"
        >
          <FontAwesomeIcon icon={faUserPlus} />
          Sign Up Free
        </Link>
      </div>
    </div>
  );
}
