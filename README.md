# Task API — Rust Coding Test

A medium-sized Rust Task API used as a **10-minute live coding exercise** for backend candidates.

## Quick start

```bash
cargo run
```

Server runs at `http://localhost:3000`.

The React board in `frontend/` renders that API. It does not implement task filtering.

```bash
cd frontend
npm install
npm run dev
```

Open http://localhost:5173 while the API is running. Status requests go to `GET /tasks`. Search, priority, and sort are browser-only.

## Candidate brief


## Time Limit

10 minutes

## Task

Implement task filtering in the Task API.

`GET /tasks` accepts an optional `status` query parameter:

- `all`
- `active`
- `completed`

### Requirements

- `all` returns every task.
- `active` returns only incomplete tasks.
- `completed` returns only completed tasks.
- Invalid or missing `status` should behave like `all` (or return a clear 400 — either is fine if consistent).
- Existing toggle behavior must continue working: `PATCH /tasks/:id/toggle`.

### Bonus

If you finish early, include a `count` field in the JSON response with the number of returned tasks.

## Rules

- Use Rust (`tiny_http` + `serde` are already configured).
- Keep the existing project structure where practical.
- Do not install unnecessary dependencies.
- Focus on functionality and clean code.

## Run

```bash
cargo run
```

Try:

```bash
curl "http://localhost:3000/tasks?status=active"
curl -X PATCH "http://localhost:3000/tasks/1/toggle"
```

## Evaluation

You will be evaluated on:

* Understanding of the existing Rust code
* Correct filtering logic
* API behavior and edge cases
* Code quality
* Ability to work independently within the time limit


## What’s already implemented

- In-memory task store with mock data
- `GET /tasks` returns tasks (filter not wired yet)
- `PATCH /tasks/:id/toggle` toggles completion
- `GET /health` health check
- CORS headers for local testing

## API

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET` | `/tasks?status=all\|active\|completed` | List tasks (candidate implements filtering) |
| `PATCH` | `/tasks/:id/toggle` | Toggle task completion |
| `GET` | `/health` | Health check |

## Stack

- Rust 2021 edition
- `tiny_http`, `serde`, `serde_json`

## Project structure

```text
src/
  data.rs        # mock tasks
  tasks.rs       # store + filter_tasks stub
  main.rs        # process entry
  app/           # HTTP server and handler
  http/          # request and response helpers
  catalog.rs     # labels and teams
  services.rs    # helpers around the in-memory store
  domain.rs      # priorities and task views
```

The exercise is still only `filter_tasks` in `src/tasks.rs`. The extra modules are supporting code and do not change the task endpoints.

## Interviewer notes

- Time box: **10 minutes**
- Core task: implement `filter_tasks` in `src/tasks.rs` for **all / active / completed**
- Optional bonus: include a `count` in the list response
- Do not share a solution with the candidate before or during the exercise
