const PRIORITY_RANK = { high: 0, medium: 1, low: 2 }

export function filterByQuery(tasks, query) {
  const needle = query.trim().toLowerCase()
  if (!needle) return tasks
  return tasks.filter((task) => task.title.toLowerCase().includes(needle))
}

export function filterByPriority(tasks, priority) {
  if (priority === 'all') return tasks
  return tasks.filter((task) => task.priority === priority)
}

export function sortTasks(tasks, sort) {
  const copy = [...tasks]
  if (sort === 'priority') {
    copy.sort(
      (left, right) =>
        (PRIORITY_RANK[left.priority] ?? 9) - (PRIORITY_RANK[right.priority] ?? 9) ||
        left.id - right.id,
    )
  } else if (sort === 'title') {
    copy.sort((left, right) => left.title.localeCompare(right.title) || left.id - right.id)
  }
  return copy
}

export function presentTasks(tasks, { query, priority, sort }) {
  return sortTasks(filterByPriority(filterByQuery(tasks, query), priority), sort)
}
