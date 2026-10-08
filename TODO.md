# Rust Coding Test

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
