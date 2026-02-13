"use client";

import { useState, useEffect, useCallback } from "react";
import {
  GameStatus,
  OpponentInfo,
  RoundResult,
  MatchResult,
  Choice,
  MoveHistoryEntry,
  RoundWinner,
} from "@/types/game";

interface UseGameStateProps {
  send: (msg: Record<string, unknown>) => void;
  addMessageHandler: (
    handler: (data: Record<string, unknown>) => void,
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
  const [myScore, setMyScore] = useState(0);
  const [opponentScore, setOpponentScore] = useState(0);
  const [moveHistory, setMoveHistory] = useState<MoveHistoryEntry[]>([]);
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
          setMyScore(0);
          setOpponentScore(0);
          setMoveHistory([]);
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
          setMyScore(data.your_score as number);
          setOpponentScore(data.opponent_score as number);
          setMoveHistory((prev) => {
            const nextEntry: MoveHistoryEntry = {
              round: data.round as number,
              playerChoice: data.your_choice as string,
              opponentChoice: data.opponent_choice as string,
              winner: data.winner as RoundWinner,
            };
            const withoutRound = prev.filter((entry) => entry.round !== nextEntry.round);
            return [...withoutRound, nextEntry].sort((a, b) => a.round - b.round);
          });
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
    [send],
  );

  const leaveQueue = useCallback(() => {
    send({ type: "leave_queue" });
    setStatus("idle");
    setMyScore(0);
    setOpponentScore(0);
    setMoveHistory([]);
  }, [send]);

  const makeChoice = useCallback(
    (choice: Choice) => {
      if (myChoice) return;
      setMyChoice(choice);
      send({ type: "choice", choice });
    },
    [send, myChoice],
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
    setMyScore(0);
    setOpponentScore(0);
    setMoveHistory([]);
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
    myScore,
    opponentScore,
    moveHistory,
    error,
    joinQueue,
    leaveQueue,
    makeChoice,
    resetGame,
  };
}
