import { useState } from "react";
import { User } from "@/types/user";
import { api } from "@/lib/api";
import UserEditModal from "./UserEditModal";
import ConfirmModal from "./ConfirmModal";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faEdit,
  faBan,
  faUserCheck,
  faTrash,
  faChevronLeft,
  faChevronRight,
} from "@fortawesome/free-solid-svg-icons";
import Image from "next/image";

interface UserManagementTableProps {
  users: User[];
  total: number;
  page: number;
  limit: number;
  search: string;
  sortBy: string;
  onSearchChange: (search: string) => void;
  onSortChange: (sortBy: string) => void;
  onPageChange: (page: number) => void;
  onRefresh: () => void;
}

export default function UserManagementTable({
  users,
  total,
  page,
  limit,
  search,
  sortBy,
  onSearchChange,
  onSortChange,
  onPageChange,
  onRefresh,
}: UserManagementTableProps) {
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [confirmAction, setConfirmAction] = useState<{
    type: "ban" | "unban" | "delete";
    user: User;
  } | null>(null);
  const [banReason, setBanReason] = useState("");
  const [loading, setLoading] = useState(false);

  const totalPages = Math.ceil(total / limit);

  const handleEdit = (user: User) => {
    setEditingUser(user);
  };

  const handleBan = async (userId: string, reason: string) => {
    setLoading(true);
    try {
      await api.post(`/api/admin/users/${userId}/ban`, { reason });
      onRefresh();
      setConfirmAction(null);
      setBanReason("");
    } catch (err) {
      console.error("Failed to ban user:", err);
      alert("Failed to ban user");
    } finally {
      setLoading(false);
    }
  };

  const handleUnban = async (userId: string) => {
    setLoading(true);
    try {
      await api.post(`/api/admin/users/${userId}/unban`, {});
      onRefresh();
      setConfirmAction(null);
    } catch (err) {
      console.error("Failed to unban user:", err);
      alert("Failed to unban user");
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (userId: string) => {
    setLoading(true);
    try {
      await api.delete(`/api/admin/users/${userId}`);
      onRefresh();
      setConfirmAction(null);
    } catch (err) {
      console.error("Failed to delete user:", err);
      alert("Failed to delete user");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-white rounded-lg border border-brand-200 overflow-hidden">
      <div className="p-4 border-b border-brand-200">
        <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
          <h2 className="font-serif text-xl font-bold text-brand-800">
            User Management
          </h2>
          <div className="flex gap-2 items-center w-full sm:w-auto">
            <input
              type="text"
              placeholder="Search users..."
              value={search}
              onChange={(e) => onSearchChange(e.target.value)}
              className="px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500 text-sm flex-1 sm:flex-none sm:w-64"
            />
            <select
              value={sortBy}
              onChange={(e) => onSortChange(e.target.value)}
              className="px-3 py-2 border border-brand-300 rounded-md focus:outline-none focus:ring-2 focus:ring-brand-500 text-sm"
            >
              <option value="created_at">Created Date</option>
              <option value="elo">Elo</option>
              <option value="total_games">Total Games</option>
            </select>
          </div>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-brand-50">
            <tr>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Username
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Email
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Elo
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Games
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Status
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-brand-700 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-brand-100">
            {users.map((user) => (
              <tr key={user.id} className="hover:bg-brand-50">
                <td className="px-4 py-3 whitespace-nowrap">
                  <div className="flex items-center">
                    {user.avatar_url && (
                      <Image
                        src={user.avatar_url}
                        alt={user.username}
                        className="h-8 w-8 rounded-full mr-2"
                      />
                    )}
                    <span className="text-sm font-medium text-brand-900">
                      {user.username}
                    </span>
                    {user.is_admin && (
                      <span className="ml-2 px-2 py-0.5 text-xs font-semibold bg-purple-100 text-purple-800 rounded">
                        Admin
                      </span>
                    )}
                  </div>
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-sm text-brand-600">
                  {user.email || "N/A"}
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-sm font-semibold text-brand-900">
                  {user.elo}
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-sm text-brand-600">
                  {user.total_games}
                </td>
                <td className="px-4 py-3 whitespace-nowrap">
                  {user.is_banned ? (
                    <span className="px-2 py-1 text-xs font-semibold bg-red-100 text-red-800 rounded">
                      Banned
                    </span>
                  ) : (
                    <span className="px-2 py-1 text-xs font-semibold bg-green-100 text-green-800 rounded">
                      Active
                    </span>
                  )}
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-sm">
                  {!user.is_admin && (
                    <div className="flex gap-2">
                      <button
                        onClick={() => handleEdit(user)}
                        className="text-brand-600 hover:text-brand-800"
                        title="Edit"
                      >
                        <FontAwesomeIcon icon={faEdit} />
                      </button>
                      {user.is_banned ? (
                        <button
                          onClick={() =>
                            setConfirmAction({ type: "unban", user })
                          }
                          className="text-green-600 hover:text-green-800"
                          title="Unban"
                        >
                          <FontAwesomeIcon icon={faUserCheck} />
                        </button>
                      ) : (
                        <button
                          onClick={() =>
                            setConfirmAction({ type: "ban", user })
                          }
                          className="text-orange-600 hover:text-orange-800"
                          title="Ban"
                        >
                          <FontAwesomeIcon icon={faBan} />
                        </button>
                      )}
                      <button
                        onClick={() =>
                          setConfirmAction({ type: "delete", user })
                        }
                        className="text-red-600 hover:text-red-800"
                        title="Delete"
                      >
                        <FontAwesomeIcon icon={faTrash} />
                      </button>
                    </div>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {totalPages > 1 && (
        <div className="px-4 py-3 border-t border-brand-200 flex items-center justify-between">
          <div className="text-sm text-brand-600">
            Showing {(page - 1) * limit + 1} to {Math.min(page * limit, total)}{" "}
            of {total} users
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => onPageChange(page - 1)}
              disabled={page === 1}
              className="px-3 py-1 border border-brand-300 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-brand-50"
            >
              <FontAwesomeIcon icon={faChevronLeft} />
            </button>
            <span className="px-3 py-1 text-sm text-brand-600">
              Page {page} of {totalPages}
            </span>
            <button
              onClick={() => onPageChange(page + 1)}
              disabled={page === totalPages}
              className="px-3 py-1 border border-brand-300 rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-brand-50"
            >
              <FontAwesomeIcon icon={faChevronRight} />
            </button>
          </div>
        </div>
      )}

      {editingUser && (
        <UserEditModal
          user={editingUser}
          onClose={() => setEditingUser(null)}
          onSuccess={() => {
            onRefresh();
            setEditingUser(null);
          }}
        />
      )}

      {confirmAction && (
        <ConfirmModal
          title={
            confirmAction.type === "ban"
              ? "Ban User"
              : confirmAction.type === "unban"
                ? "Unban User"
                : "Delete User"
          }
          message={
            confirmAction.type === "ban"
              ? `Are you sure you want to ban ${confirmAction.user.username}? They will not be able to log in.`
              : confirmAction.type === "unban"
                ? `Are you sure you want to unban ${confirmAction.user.username}?`
                : `Are you sure you want to permanently delete ${confirmAction.user.username}? This action cannot be undone.`
          }
          confirmText={
            confirmAction.type === "ban"
              ? "Ban"
              : confirmAction.type === "unban"
                ? "Unban"
                : "Delete"
          }
          confirmColor={
            confirmAction.type === "unban" ? "bg-green-600" : "bg-red-600"
          }
          onConfirm={() => {
            if (confirmAction.type === "ban") {
              handleBan(confirmAction.user.id, banReason);
            } else if (confirmAction.type === "unban") {
              handleUnban(confirmAction.user.id);
            } else {
              handleDelete(confirmAction.user.id);
            }
          }}
          onCancel={() => {
            setConfirmAction(null);
            setBanReason("");
          }}
          loading={loading}
          showReasonInput={confirmAction.type === "ban"}
          reason={banReason}
          onReasonChange={setBanReason}
        />
      )}
    </div>
  );
}
