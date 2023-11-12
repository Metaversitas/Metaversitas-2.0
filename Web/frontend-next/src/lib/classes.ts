import fetchInstance from '@/lib/fetch-instance'

export const getAvailableClasses = async () => {
  return await fetchInstance('/classroom')
}
