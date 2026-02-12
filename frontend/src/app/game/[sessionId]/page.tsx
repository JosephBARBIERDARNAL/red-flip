"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function GameSessionPage() {
  const router = useRouter();

  useEffect(() => {
    // Game sessions are handled on the /play page via WebSocket
    router.replace("/play");
  }, [router]);

  return null;
}
