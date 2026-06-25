import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { reviewApi } from '../api/reviews'

export interface DailyReview {
  date: string
  content: string
  mood: 'productive' | 'normal' | 'slacking'
  linkedTodoIds: string[]
  createdAt: string
  updatedAt: string
}

export const useReviewStore = defineStore('reviews', () => {
  const reviews = ref<DailyReview[]>([])
  const loading = ref(false)

  const todayReview = computed(() => {
    const today = new Date().toISOString().slice(0, 10)
    return reviews.value.find(r => r.date === today) || null
  })

  const recentReviews = computed(() => {
    return [...reviews.value]
      .sort((a, b) => b.date.localeCompare(a.date))
      .slice(0, 14)
  })

  async function fetchReviews(year?: number, month?: number) {
    loading.value = true
    try {
      reviews.value = await reviewApi.list(year, month)
    } catch (e) {
      console.error('fetchReviews failed:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveReview(data: { content: string; mood: string; linkedTodoIds?: string[] }) {
    try {
      const review = await reviewApi.save({
        content: data.content,
        mood: data.mood,
        linkedTodoIds: data.linkedTodoIds || [],
      })
      const idx = reviews.value.findIndex(r => r.date === review.date)
      if (idx !== -1) {
        reviews.value[idx] = review
      } else {
        reviews.value.unshift(review)
      }
      return review
    } catch (e) {
      console.error('saveReview failed:', e)
      return null
    }
  }

  async function getReview(date: string) {
    try {
      return await reviewApi.get(date)
    } catch (e) {
      console.error('getReview failed:', e)
      return null
    }
  }

  return {
    reviews,
    loading,
    todayReview,
    recentReviews,
    fetchReviews,
    saveReview,
    getReview,
  }
})
