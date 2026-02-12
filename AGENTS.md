# Red Flip - Agent Instructions

## Project Overview

Full-stack Rock Paper Scissors web app with Elo ranking. Monorepo with `backend/` (Rust/Actix-Web) and `frontend/` (Next.js/TypeScript).

## Build Commands

- Backend: `cd backend && cargo build`
- Frontend: `cd frontend && npm run build`
- Run backend: `cd backend && cargo run` (needs `.env` with `JWT_SECRET`)
- Run frontend: `cd frontend && npm run dev`

## Architecture

- Backend uses Actix actor model: MatchmakingActor (singleton), GameSessionActor (per match), PlayerWsActor (per WebSocket connection)
- Frontend uses Next.js App Router with client-side auth (JWT in localStorage)
- Turso database
- WebSocket at `/ws?token=JWT` for real-time game communication

## Key Design Rules

- Brand color: `#9d0208` (brand-600)
- Fonts: Open Sans (body), Noto Serif (headings), Coming Soon (accents)
- No emojis - use Font Awesome icons only
- No hover effects on non-clickable elements
- Best-of-3 game format, 15-second round timer
- Elo K-factors: 40 (new), 20 (standard), 10 (2400+)

## File Organization

- `backend/src/models/` - Database models with query methods
- `backend/src/auth/` - JWT, middleware, handlers, Google OAuth
- `backend/src/api/` - REST endpoint handlers
- `backend/src/game/` - Elo calc, matchmaking, game session, WebSocket actors
- `frontend/src/components/` - UI components organized by feature
- `frontend/src/hooks/` - React hooks (auth, WebSocket, game state)
- `frontend/src/context/` - Auth context provider
- `frontend/src/types/` - TypeScript type definitions
