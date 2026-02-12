import { useState } from "react";
import { User } from "@/types/user";
import { UpdateUserRequest } from "@/types/admin";
import { api } from "@/lib/api";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTimes, faSpinner } from "@fortawesome/free-solid-svg-icons";

interface UserEditModalProps {
  user: User;
  onClose: () => void;
  onSuccess: () => void;
}

export default function UserEditModal({
  user,
  onClose,
  onSuccess,
}: UserEditModalProps) {
  const [formData, setFormData] = useState({
    username: user.username,
    elo: user.elo.toString(),
    wins: user.wins.toString(),
    losses: user.losses.toString(),
    draws: user.draws.toString(),
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      const updates: UpdateUserRequest = {};

      if (formData.username !== user.username) {
        updates.username = formData.username;
      }
      if (parseInt(formData.elo) !== user.elo) {
        updates.elo = parseInt(formData.elo);
      }
      if (parseInt(formData.wins) !== user.wins) {
        updates.wins = parseInt(formData.wins);
      }
      if (parseInt(formData.losses) !== user.losses) {
        updates.losses = parseInt(formData.losses);
      }
      if (parseInt(formData.draws) !== user.draws) {
        updates.draws = parseInt(formData.draws);
      }

      if (Object.keys(updates).length === 0) {
        onClose();
        return;
      }

      await api.put(`/api/admin/users/${user.id}`, updates);
      onSuccess();
    } catch (err: any) {
      setError(err.message || "Failed to update user");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl max-w-md w-full">
        <div className="flex items-center justify-between p-4 border-b border-brand-200">
          <h3 className="font-serif text-xl font-bold text-brand-800">
            Edit User: {user.username}
          </h3>
          <button
            onClick={onClose}
            className="text-brand-400 hover:text-brand-600"
          >
            <FontAwesomeIcon icon={faTimes} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          {error && (
            <div className="p-3 bg-red-50 border border-red-200 text-red-700 rounded-md text-sm">
              {error}
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-brand-700 mb-1">
              Username
            </label>
            <input
              type="text"
              value={formData.username}
              onChange={(e) =>
                setFormData({ ...formData, username: e.target.value })
              }
              className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
              required
              minLength={3}
              maxLength={20}
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-brand-700 mb-1">
              Elo
            </label>
            <input
              type="number"
              value={formData.elo}
              onChange={(e) =>
                setFormData({ ...formData, elo: e.target.value })
              }
              className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
              required
              min={0}
              max={5000}
            />
          </div>

          <div className="grid grid-cols-3 gap-3">
            <div>
              <label className="block text-sm font-medium text-brand-700 mb-1">
                Wins
              </label>
              <input
                type="number"
                value={formData.wins}
                onChange={(e) =>
                  setFormData({ ...formData, wins: e.target.value })
                }
                className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
                required
                min={0}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-brand-700 mb-1">
                Losses
              </label>
              <input
                type="number"
                value={formData.losses}
                onChange={(e) =>
                  setFormData({ ...formData, losses: e.target.value })
                }
                className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
                required
                min={0}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-brand-700 mb-1">
                Draws
              </label>
              <input
                type="number"
                value={formData.draws}
                onChange={(e) =>
                  setFormData({ ...formData, draws: e.target.value })
                }
                className="w-full px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500"
                required
                min={0}
              />
            </div>
          </div>

          <div className="flex gap-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 px-4 py-2 border border-brand-300 text-brand-700 rounded-md hover:bg-brand-50"
              disabled={loading}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="flex-1 px-4 py-2 bg-brand-600 text-white rounded-md hover:bg-brand-700 disabled:opacity-50"
              disabled={loading}
            >
              {loading ? (
                <FontAwesomeIcon icon={faSpinner} spin />
              ) : (
                "Save Changes"
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
