import { WS_BASE_URL } from "./constants";

export function createGameSocket(token: string): WebSocket {
  return new WebSocket(`${WS_BASE_URL}/ws?token=${token}`);
}
