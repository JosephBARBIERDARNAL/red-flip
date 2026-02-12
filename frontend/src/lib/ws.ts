import { WS_BASE_URL } from "./constants";

export function createGameSocket(token: string | null): WebSocket {
  const url = token ? `${WS_BASE_URL}/ws?token=${token}` : `${WS_BASE_URL}/ws`;
  return new WebSocket(url);
}
