"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/hooks/useAuth";
import { api } from "@/lib/api";
import { AdminStatsResponse, AdminUsersResponse } from "@/types/admin";
import AdminStats from "@/components/admin/AdminStats";
import UserManagementTable from "@/components/admin/UserManagementTable";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSpinner } from "@fortawesome/free-solid-svg-icons";

export default function AdminPage() {
  const { user, loading: authLoading } = useAuth();
  const router = useRouter();
  const [stats, setStats] = useState<AdminStatsResponse | null>(null);
  const [usersData, setUsersData] = useState<AdminUsersResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState("");
  const [sortBy, setSortBy] = useState("created_at");
  const [page, setPage] = useState(1);

  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
      return;
    }

    if (!authLoading && user && !user.is_admin) {
      router.push("/dashboard");
      return;
    }
  }, [user, authLoading, router]);

  useEffect(() => {
    if (!user?.is_admin) return;

    const fetchAdminData = async () => {
      setLoading(true);
      try {
        const [statsRes, usersRes] = await Promise.all([
          api.get<AdminStatsResponse>("/api/admin/stats"),
          api.get<AdminUsersResponse>(
            `/api/admin/users?search=${search}&sort_by=${sortBy}&page=${page}&limit=20`,
          ),
        ]);
        setStats(statsRes);
        setUsersData(usersRes);
      } catch (err) {
        console.error("Failed to fetch admin data:", err);
        if (
          err instanceof Error &&
          (err.message.includes("401") || err.message.includes("Admin"))
        ) {
          router.push("/dashboard");
        }
      } finally {
        setLoading(false);
      }
    };

    void fetchAdminData();
  }, [user, search, sortBy, page, router]);

  const handleRefreshUsers = () => {
    if (!user?.is_admin) return;

    api
      .get<AdminUsersResponse>(
        `/api/admin/users?search=${search}&sort_by=${sortBy}&page=${page}&limit=20`,
      )
      .then(setUsersData)
      .catch(console.error);
  };

  if (authLoading || loading) {
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

  if (!user?.is_admin || !stats || !usersData) return null;

  return (
    <div className="max-w-7xl mx-auto py-8 px-4">
      <h1 className="font-hand text-3xl font-bold text-brand-800 mb-8">
        Admin Dashboard
      </h1>

      <AdminStats stats={stats.stats} />

      <div className="mt-8">
        <UserManagementTable
          users={usersData.users}
          total={usersData.total}
          page={usersData.page}
          limit={usersData.limit}
          search={search}
          sortBy={sortBy}
          onSearchChange={setSearch}
          onSortChange={setSortBy}
          onPageChange={setPage}
          onRefresh={handleRefreshUsers}
        />
      </div>
    </div>
  );
}
