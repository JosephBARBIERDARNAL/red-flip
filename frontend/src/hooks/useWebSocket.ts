"use client";

import { useEffect, useRef, useCallback, useState } from "react";
import { createGameSocket } from "@/lib/ws";

type MessageHandler = (data: Record<string, unknown>) => void;

export function useWebSocket(token: string | null, allowGuest = false) {
  const wsRef = useRef<WebSocket | null>(null);
  const [connected, setConnected] = useState(false);
  const handlersRef = useRef<MessageHandler[]>([]);

  const addMessageHandler = useCallback((handler: MessageHandler) => {
    handlersRef.current.push(handler);
    return () => {
      handlersRef.current = handlersRef.current.filter((h) => h !== handler);
    };
  }, []);

  const send = useCallback((msg: Record<string, unknown>) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  useEffect(() => {
    // Only connect if we have a token OR guest mode is explicitly enabled
    // This prevents connection attempts during initial auth loading
    if (!token && !allowGuest) return;

    const ws = createGameSocket(token);
    wsRef.current = ws;

    ws.onopen = () => setConnected(true);
    ws.onclose = () => setConnected(false);
    ws.onerror = () => setConnected(false);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        handlersRef.current.forEach((handler) => handler(data));
      } catch {
        // ignore invalid messages
      }
    };

    return () => {
      ws.close();
      wsRef.current = null;
      setConnected(false);
    };
  }, [token, allowGuest]);

  return { connected, send, addMessageHandler };
}
