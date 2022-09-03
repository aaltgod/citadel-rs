run: build
	cd backend && cargo run --release

build:
	cd frontend && trunk build -d ../backend/static/