"use client";

import Link from "next/link";
import { useAuth } from "@/hooks/useAuth";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faHandFist,
  faUser,
  faTrophy,
  faGamepad,
  faSignOutAlt,
  faCog,
  faShieldAlt,
} from "@fortawesome/free-solid-svg-icons";

export default function Header() {
  const { user, logout, loading } = useAuth();

  return (
    <header className="bg-brand-700 text-white shadow-lg">
      <div className="max-w-6xl mx-auto px-4 py-3 flex items-center justify-between">
        <Link
          href="/"
          className="flex items-center gap-2 font-serif text-xl font-bold"
        >
          <FontAwesomeIcon icon={faHandFist} className="text-brand-300" />
          Red Flip
        </Link>

        <nav className="flex items-center gap-4">
          {loading ? null : user ? (
            <>
              <Link
                href="/play"
                className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-brand-500 hover:bg-brand-400 transition-colors text-sm font-medium"
              >
                <FontAwesomeIcon icon={faGamepad} />
                Play
              </Link>
              <Link
                href="/leaderboard"
                className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm"
              >
                <FontAwesomeIcon icon={faTrophy} />
                Leaderboard
              </Link>
              {user.is_admin && (
                <Link
                  href="/admin"
                  className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm"
                >
                  <FontAwesomeIcon icon={faShieldAlt} />
                  Admin
                </Link>
              )}
              <Link
                href="/dashboard"
                className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm"
              >
                <FontAwesomeIcon icon={faUser} />
                {user.username}
              </Link>
              <Link
                href="/settings"
                className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm"
              >
                <FontAwesomeIcon icon={faCog} />
              </Link>
              <button
                onClick={logout}
                className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm cursor-pointer"
              >
                <FontAwesomeIcon icon={faSignOutAlt} />
              </button>
            </>
          ) : (
            <>
              <Link
                href="/leaderboard"
                className="flex items-center gap-1.5 hover:text-brand-200 transition-colors text-sm"
              >
                <FontAwesomeIcon icon={faTrophy} />
                Leaderboard
              </Link>
              <Link
                href="/login"
                className="px-3 py-1.5 rounded border border-brand-300 hover:bg-brand-600 transition-colors text-sm"
              >
                Log In
              </Link>
              <Link
                href="/register"
                className="px-3 py-1.5 rounded bg-brand-500 hover:bg-brand-400 transition-colors text-sm font-medium"
              >
                Sign Up
              </Link>
            </>
          )}
        </nav>
      </div>
    </header>
  );
}
