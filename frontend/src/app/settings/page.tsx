"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/hooks/useAuth";
import { api } from "@/lib/api";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSpinner,
  faTrashAlt,
  faExclamationTriangle,
} from "@fortawesome/free-solid-svg-icons";

export default function SettingsPage() {
  const { user, loading: authLoading, logout } = useAuth();
  const router = useRouter();
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
    }
  }, [user, authLoading, router]);

  const handleDeleteAccount = async () => {
    setError(null);
    setDeleting(true);

    try {
      await api.delete("/api/account/delete");
      logout();
      router.push("/");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete account");
      setDeleting(false);
    }
  };

  if (authLoading) {
    return (
      <div className="flex justify-center py-20">
        <FontAwesomeIcon
          icon={faSpinner}
          spin
          className="text-brand-600 text-3xl"
        />
      </div>
    );
  }

  if (!user) return null;

  return (
    <div className="max-w-2xl mx-auto py-8 px-4">
      <h1 className="font-serif text-3xl font-bold text-brand-800 mb-8">
        Account Settings
      </h1>

      <div className="bg-white rounded-lg shadow-md p-6 mb-6">
        <h2 className="font-serif text-xl font-bold text-brand-800 mb-4">
          Profile Information
        </h2>
        <div className="space-y-3">
          <div>
            <label className="text-sm text-gray-600">Username</label>
            <p className="font-medium">{user.username}</p>
          </div>
          <div>
            <label className="text-sm text-gray-600">Elo Rating</label>
            <p className="font-medium">{user.elo}</p>
          </div>
          <div>
            <label className="text-sm text-gray-600">Total Games</label>
            <p className="font-medium">{user.total_games}</p>
          </div>
        </div>
      </div>

      <div className="bg-red-50 rounded-lg shadow-md p-6 border border-red-200">
        <h2 className="font-serif text-xl font-bold text-red-800 mb-4">
          Danger Zone
        </h2>
        <p className="text-gray-700 mb-4">
          Once you delete your account, there is no going back. This will
          permanently delete your account, including all your match history and
          statistics.
        </p>
        <button
          onClick={() => setShowDeleteModal(true)}
          className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors font-medium"
        >
          <FontAwesomeIcon icon={faTrashAlt} />
          Delete My Account
        </button>
      </div>

      {showDeleteModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
            <div className="flex items-center gap-3 mb-4">
              <FontAwesomeIcon
                icon={faExclamationTriangle}
                className="text-red-600 text-2xl"
              />
              <h3 className="font-serif text-xl font-bold text-gray-900">
                Delete Account
              </h3>
            </div>

            <p className="text-gray-700 mb-6">
              Are you absolutely sure you want to delete your account? This
              action cannot be undone. All your data, including match history,
              statistics, and Elo rating will be permanently deleted.
            </p>

            {error && (
              <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded text-red-700 text-sm">
                {error}
              </div>
            )}

            <div className="flex gap-3 justify-end">
              <button
                onClick={() => {
                  setShowDeleteModal(false);
                  setError(null);
                }}
                disabled={deleting}
                className="px-4 py-2 border border-gray-300 rounded hover:bg-gray-50 transition-colors font-medium disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                onClick={handleDeleteAccount}
                disabled={deleting}
                className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors font-medium flex items-center gap-2 disabled:opacity-50"
              >
                {deleting ? (
                  <>
                    <FontAwesomeIcon icon={faSpinner} spin />
                    Deleting...
                  </>
                ) : (
                  <>
                    <FontAwesomeIcon icon={faTrashAlt} />
                    Yes, Delete My Account
                  </>
                )}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
