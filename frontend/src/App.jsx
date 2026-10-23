import { useCallback, useEffect, useMemo, useState } from 'react'
import { fetchHealth, fetchTasks, toggleTask } from './api'
import { presentTasks } from './taskView'

const STATUSES = [
  { id: 'all', label: 'All' },
  { id: 'active', label: 'Active' },
  { id: 'completed', label: 'Completed' },
]

const PRIORITIES = [
  { id: 'all', label: 'Any priority' },
  { id: 'high', label: 'High' },
  { id: 'medium', label: 'Medium' },
  { id: 'low', label: 'Low' },
]

const PRIORITY_STYLES = {
  high: 'bg-orange-100 text-orange-950',
  medium: 'bg-amber-100 text-amber-950',
  low: 'bg-stone-200 text-stone-700',
}

function App() {
  const [status, setStatus] = useState('all')
  const [query, setQuery] = useState('')
  const [priority, setPriority] = useState('all')
  const [sort, setSort] = useState('api')
  const [tasks, setTasks] = useState([])
  const [count, setCount] = useState(0)
  const [healthy, setHealthy] = useState(false)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [pendingId, setPendingId] = useState(null)
  const [reloadToken, setReloadToken] = useState(0)

  const reload = useCallback(() => {
    setLoading(true)
    setError('')
    setReloadToken((value) => value + 1)
  }, [])

  function changeStatus(next) {
    setStatus(next)
    setLoading(true)
    setError('')
  }

  useEffect(() => {
    let cancelled = false

    Promise.all([fetchTasks(status), fetchHealth().then(() => true).catch(() => false)])
      .then(([result, ok]) => {
        if (cancelled) return
        setTasks(result.tasks)
        setCount(result.count)
        setHealthy(ok)
      })
      .catch((err) => {
        if (cancelled) return
        setTasks([])
        setCount(0)
        setHealthy(false)
        setError(err instanceof Error ? err.message : 'Could not reach the Task API')
      })
      .finally(() => {
        if (!cancelled) setLoading(false)
      })

    return () => {
      cancelled = true
    }
  }, [status, reloadToken])

  const visibleTasks = useMemo(
    () => presentTasks(tasks, { query, priority, sort }),
    [tasks, query, priority, sort],
  )

  async function handleToggle(id) {
    setPendingId(id)
    setError('')
    try {
      await toggleTask(id)
      const result = await fetchTasks(status)
      setTasks(result.tasks)
      setCount(result.count)
      setHealthy(true)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not update task')
    } finally {
      setPendingId(null)
    }
  }

  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-6 px-4 py-8 sm:px-6 sm:py-12">
      <header className="flex flex-wrap items-end justify-between gap-4">
        <div>
          <p className="text-xs font-semibold tracking-[0.18em] text-pine uppercase">Task API</p>
          <h1 className="mt-1 font-display text-4xl tracking-tight text-ink">Task board</h1>
          <p className="mt-2 max-w-xl text-sm leading-6 text-stone-600">
            Status is loaded from the Rust API. Search, priority, and sort stay in the browser.
          </p>
        </div>
        <p
          className={`inline-flex items-center gap-2 rounded-full border px-3 py-1 text-sm ${
            healthy ? 'border-pine/30 bg-pine/10 text-pine' : 'border-clay/30 bg-orange-50 text-clay'
          }`}
        >
          <span className={`size-2 rounded-full ${healthy ? 'bg-pine' : 'bg-clay'}`} />
          {healthy ? 'API healthy' : 'API unreachable'}
        </p>
      </header>

      <section className="rounded-2xl border border-line bg-card shadow-[0_16px_40px_rgb(28_25_21_/_0.05)]">
        <div className="flex flex-col gap-4 border-b border-line p-4 sm:p-5">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="inline-flex rounded-full bg-paper p-1" role="radiogroup" aria-label="Task status">
              {STATUSES.map((item) => {
                const selected = status === item.id
                return (
                  <button
                    key={item.id}
                    type="button"
                    role="radio"
                    aria-checked={selected}
                    onClick={() => changeStatus(item.id)}
                    className={`rounded-full px-3 py-1.5 text-sm font-medium transition ${
                      selected ? 'bg-ink text-card' : 'text-stone-600 hover:text-ink'
                    }`}
                  >
                    {item.label}
                  </button>
                )
              })}
            </div>
            <button
              type="button"
              onClick={reload}
              className="rounded-full border border-line px-3 py-1.5 text-sm text-stone-700 hover:border-stone-400"
            >
              Refresh
            </button>
          </div>

          <div className="grid gap-3 sm:grid-cols-[1fr_10rem_10rem]">
            <label className="block">
              <span className="mb-1 block text-xs font-medium tracking-wide text-stone-500 uppercase">
                Search
              </span>
              <input
                value={query}
                onChange={(event) => setQuery(event.target.value)}
                placeholder="Find a title"
                className="w-full rounded-xl border border-line bg-white px-3 py-2 text-sm outline-none focus:border-pine"
              />
            </label>
            <label className="block">
              <span className="mb-1 block text-xs font-medium tracking-wide text-stone-500 uppercase">
                Priority
              </span>
              <select
                value={priority}
                onChange={(event) => setPriority(event.target.value)}
                className="w-full rounded-xl border border-line bg-white px-3 py-2 text-sm outline-none focus:border-pine"
              >
                {PRIORITIES.map((item) => (
                  <option key={item.id} value={item.id}>
                    {item.label}
                  </option>
                ))}
              </select>
            </label>
            <label className="block">
              <span className="mb-1 block text-xs font-medium tracking-wide text-stone-500 uppercase">
                Sort
              </span>
              <select
                value={sort}
                onChange={(event) => setSort(event.target.value)}
                className="w-full rounded-xl border border-line bg-white px-3 py-2 text-sm outline-none focus:border-pine"
              >
                <option value="api">API order</option>
                <option value="priority">Priority</option>
                <option value="title">Title</option>
              </select>
            </label>
          </div>

          <p className="text-sm text-stone-500">
            {loading ? (
              'Loading tasks…'
            ) : (
              <>
                {count} returned for <span className="font-medium text-ink">{status}</span>
                {visibleTasks.length === count ? '' : ` · ${visibleTasks.length} shown`}
              </>
            )}
          </p>
        </div>

        {error ? (
          <p className="m-4 rounded-xl bg-orange-50 px-4 py-3 text-sm text-clay" role="alert">
            {error}. Start the API with <code className="font-mono">cargo run</code> on port 3000.
          </p>
        ) : null}

        {loading ? null : visibleTasks.length === 0 ? (
          <p className="px-5 py-8 text-sm text-stone-500">
            {tasks.length === 0 ? 'No tasks came back for this status.' : 'Nothing matches the current search or priority.'}
          </p>
        ) : (
          <ul>
            {visibleTasks.map((task) => (
              <li
                key={task.id}
                className="flex items-center gap-3 border-b border-line px-4 py-3 last:border-b-0 sm:px-5"
              >
                <button
                  type="button"
                  aria-pressed={task.completed}
                  aria-label={
                    task.completed ? `Mark ${task.title} active` : `Mark ${task.title} completed`
                  }
                  disabled={pendingId === task.id}
                  onClick={() => handleToggle(task.id)}
                  className={`grid size-6 shrink-0 place-items-center rounded-full border text-xs ${
                    task.completed
                      ? 'border-pine bg-pine text-white'
                      : 'border-stone-300 bg-white text-transparent hover:border-pine'
                  } disabled:opacity-50`}
                >
                  ✓
                </button>
                <div className="min-w-0 flex-1">
                  <p className={task.completed ? 'truncate text-stone-500 line-through' : 'truncate'}>
                    {task.title}
                  </p>
                  <p className="text-xs text-stone-500">Task {task.id}</p>
                </div>
                <span
                  className={`rounded-full px-2.5 py-1 text-xs font-medium capitalize ${
                    PRIORITY_STYLES[task.priority] ?? 'bg-stone-100 text-stone-700'
                  }`}
                >
                  {task.priority}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  )
}

export default App
