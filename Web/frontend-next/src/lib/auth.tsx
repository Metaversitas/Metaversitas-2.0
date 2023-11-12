import axiosInstance from '@/lib/axios-instance'

export const getRefreshToken = async () => {
  const response = await axiosInstance.get('/auth/refresh')
  return response.data
}
