dev-backend:
    cd backend && cargo run

dev-frontend:
    cd frontend && npm run dev

dev:
    @echo "Run 'just dev-backend' and 'just dev-frontend' in separate terminals"

back:
    cd backend && cargo build

front:
    cd frontend && npm run build

build: back front
