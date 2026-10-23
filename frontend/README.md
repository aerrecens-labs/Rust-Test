# Task board

React and Tailwind UI for the Rust Task API. Status filtering stays in `src/tasks.rs`. This page only sends `status` to `GET /tasks` and toggles completion with `PATCH /tasks/:id/toggle`.

Search, priority, and sort run in the browser on whatever the API returns.

## Run

Start the API from the repository root:

```bash
cargo run
```

Then, in this folder:

```bash
npm install
npm run dev
```

Open http://localhost:5173. Vite proxies `/tasks` and `/health` to http://127.0.0.1:3000.
