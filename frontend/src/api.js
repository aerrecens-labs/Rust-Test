export async function fetchHealth() {
  const response = await fetch('/health')
  if (!response.ok) {
    throw new Error('Health check failed')
  }
  return response.json()
}

export async function fetchTasks(status) {
  const response = await fetch(`/tasks?status=${encodeURIComponent(status)}`)
  if (!response.ok) {
    throw new Error(`Could not load tasks (${response.status})`)
  }

  const body = await response.json()
  if (Array.isArray(body)) {
    return { tasks: body, count: body.length }
  }

  const tasks = Array.isArray(body.tasks) ? body.tasks : []
  const count = typeof body.count === 'number' ? body.count : tasks.length
  return { tasks, count }
}

export async function toggleTask(id) {
  const response = await fetch(`/tasks/${id}/toggle`, { method: 'PATCH' })
  if (response.status === 404) {
    throw new Error('Task not found')
  }
  if (!response.ok) {
    throw new Error(`Could not update task (${response.status})`)
  }
  return response.json()
}
