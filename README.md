# Red Flip

Real-time Rock Paper Scissors with Elo ranking. Play best-of-3 matches against live opponents, climb the leaderboard, and track your stats.

## Tech Stack

- **Backend**: Rust (Actix-Web) + SQLite (sqlx) + WebSockets (actix actors)
- **Frontend**: Next.js (App Router) + Tailwind CSS + TypeScript

## Setup

```bash
# Backend
cd backend
cp ../.env.example .env
# Edit .env with your JWT_SECRET
cargo run

# Frontend (in another terminal)
cd frontend
npm install
npm run dev
```

Backend runs on `http://localhost:8080`, frontend on `http://localhost:3000`.

## Environment Variables

See `.env.example` for all configuration options. Required: `JWT_SECRET`.

## Game Rules

- Best-of-3 format (first to 2 round wins)
- 15-second timer per round
- Draws don't count toward score, max 5 rounds
- Disconnecting mid-game forfeits the match
- Elo system: K=40 (<30 games), K=20 (standard), K=10 (2400+ Elo)

## API Endpoints

### Auth

- `POST /auth/register` - Create account
- `POST /auth/login` - Log in
- `GET /auth/me` - Current user (requires auth)
- `GET /auth/google` - Google OAuth redirect
- `GET /auth/google/callback` - Google OAuth callback

### API

- `GET /api/health` - Health check
- `GET /api/leaderboard` - Top 10 players
- `GET /api/dashboard` - User stats + recent matches (requires auth)
- `GET /api/users/:id` - Public user profile

### WebSocket

- `GET /ws?token=JWT` - Game WebSocket connection
