import { tauriInvoke } from './adapter'
import type { DailyReview } from '../stores/useReviewStore'

export const reviewApi = {
  list: (year?: number, month?: number) =>
    tauriInvoke<DailyReview[]>('list_reviews', { year: year || null, month: month || null }),

  get: (date: string) =>
    tauriInvoke<DailyReview | null>('get_review', { date }),

  save: (data: { content: string; mood: string; linkedTodoIds?: string[] }) =>
    tauriInvoke<DailyReview>('save_review', {
      content: data.content,
      mood: data.mood,
      linkedTodoIds: data.linkedTodoIds || [],
    }),

  generateWeeklySummary: () =>
    tauriInvoke<string>('generate_weekly_summary', {}),
}
