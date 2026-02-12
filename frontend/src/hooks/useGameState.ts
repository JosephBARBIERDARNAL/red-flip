"use client";

import { useState, useEffect, useCallback } from "react";
import {
  GameStatus,
  OpponentInfo,
  RoundResult,
  MatchResult,
  Choice,
} from "@/types/game";

interface UseGameStateProps {
  send: (msg: Record<string, unknown>) => void;
  addMessageHandler: (
    handler: (data: Record<string, unknown>) => void
  ) => () => void;
}

export function useGameState({ send, addMessageHandler }: UseGameStateProps) {
  const [status, setStatus] = useState<GameStatus>("idle");
  const [opponent, setOpponent] = useState<OpponentInfo | null>(null);
  const [currentRound, setCurrentRound] = useState(1);
  const [timeLeft, setTimeLeft] = useState(15);
  const [opponentChose, setOpponentChose] = useState(false);
  const [roundResult, setRoundResult] = useState<RoundResult | null>(null);
  const [matchResult, setMatchResult] = useState<MatchResult | null>(null);
  const [myChoice, setMyChoice] = useState<Choice | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const remove = addMessageHandler((data) => {
      const type = data.type as string;

      switch (type) {
        case "queued":
          setStatus("queued");
          break;
        case "match_found":
          setStatus("playing");
          setOpponent(data.opponent as OpponentInfo);
          setCurrentRound(1);
          setMyChoice(null);
          setOpponentChose(false);
          break;
        case "round_start":
          setCurrentRound(data.round as number);
          setTimeLeft(data.timeout_secs as number);
          setMyChoice(null);
          setOpponentChose(false);
          setRoundResult(null);
          setStatus("playing");
          break;
        case "opponent_chose":
          setOpponentChose(true);
          break;
        case "round_result":
          setRoundResult(data as unknown as RoundResult);
          setStatus("round_result");
          break;
        case "match_complete":
          setMatchResult(data as unknown as MatchResult);
          setStatus("match_complete");
          break;
        case "opponent_disconnected":
          setError("Opponent disconnected");
          break;
        case "error":
          setError(data.message as string);
          break;
      }
    });

    return remove;
  }, [addMessageHandler]);

  // Countdown timer
  useEffect(() => {
    if (status !== "playing" || timeLeft <= 0) return;

    const interval = setInterval(() => {
      setTimeLeft((t) => {
        if (t <= 1) {
          clearInterval(interval);
          return 0;
        }
        return t - 1;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [status, timeLeft]);

  const joinQueue = useCallback(
    (ranked = true) => {
      send({ type: "join_queue", ranked });
    },
    [send]
  );

  const leaveQueue = useCallback(() => {
    send({ type: "leave_queue" });
    setStatus("idle");
  }, [send]);

  const makeChoice = useCallback(
    (choice: Choice) => {
      if (myChoice) return;
      setMyChoice(choice);
      send({ type: "choice", choice });
    },
    [send, myChoice]
  );

  const resetGame = useCallback(() => {
    setStatus("idle");
    setOpponent(null);
    setCurrentRound(1);
    setTimeLeft(15);
    setOpponentChose(false);
    setRoundResult(null);
    setMatchResult(null);
    setMyChoice(null);
    setError(null);
  }, []);

  return {
    status,
    opponent,
    currentRound,
    timeLeft,
    opponentChose,
    roundResult,
    matchResult,
    myChoice,
    error,
    joinQueue,
    leaveQueue,
    makeChoice,
    resetGame,
  };
}
