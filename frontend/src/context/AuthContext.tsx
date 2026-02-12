"use client";

import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  ReactNode,
} from "react";
import { User } from "@/types/user";
import { api } from "@/lib/api";
import { MeResponse, AuthResponse } from "@/types/api";

interface AuthContextValue {
  user: User | null;
  token: string | null;
  loading: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (
    username: string,
    email: string,
    password: string,
  ) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const saveToken = (t: string) => {
    localStorage.setItem("token", t);
    setToken(t);
  };

  const clearAuth = () => {
    localStorage.removeItem("token");
    setToken(null);
    setUser(null);
  };

  const fetchUser = useCallback(async () => {
    try {
      const data = await api.get<MeResponse>("/auth/me");
      setUser(data.user);
    } catch {
      clearAuth();
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    // Check for token in URL (Google OAuth redirect)
    const params = new URLSearchParams(window.location.search);
    const urlToken = params.get("token");
    if (urlToken) {
      saveToken(urlToken);
      window.history.replaceState({}, "", window.location.pathname);
    }

    const stored = urlToken || localStorage.getItem("token");
    if (stored) {
      setToken(stored);
      if (urlToken) saveToken(urlToken);
      fetchUser();
    } else {
      setLoading(false);
    }
  }, [fetchUser]);

  const login = async (email: string, password: string) => {
    const data = await api.post<AuthResponse>("/auth/login", {
      email,
      password,
    });
    saveToken(data.token);
    setUser(data.user);
  };

  const register = async (
    username: string,
    email: string,
    password: string,
  ) => {
    const data = await api.post<AuthResponse>("/auth/register", {
      username,
      email,
      password,
    });
    saveToken(data.token);
    setUser(data.user);
  };

  const logout = () => {
    clearAuth();
  };

  return (
    <AuthContext.Provider
      value={{ user, token, loading, login, register, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
