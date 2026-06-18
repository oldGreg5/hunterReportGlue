# Hunter Report Generator (Rust Version)

A high-performance report generator built with Rust, Yew (frontend), and Axum (backend).

## Features
- **Fast & Safe:** Built with Rust for memory safety and performance.
- **WebAssembly:** Frontend compiled to WASM using Yew.
- **DOCX Generation:** Server-side `.docx` file generation.
- **Wizard Flow:** Easy-to-use step-by-step process.

## Prerequisites
- Rust (latest stable)
- `wasm-pack` (for building the frontend)
- `cargo`

## Project Structure
- `backend/`: Axum server that handles API requests and serves static files.
- `frontend/`: Yew-based WASM application.
- `static/`: Static assets, including the compiled frontend and data files.
- `data/`: Raw data files.

## Development Setup

### Build the Frontend
Navigate to the `frontend` directory and run:
```bash
wasm-pack build --target web --out-dir ../static/hunterReport/pkg
```

### Start the Backend
From the project root:
```bash
cargo run --bin hunter_report_backend
```

### Access the application
Once the server is running, open your browser and go to:
[http://127.0.0.1:8081/hunterReport](http://127.0.0.1:8081/hunterReport)

## License
ISC
