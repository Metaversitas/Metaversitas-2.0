import axiosInstance from '@/lib/axios-instance'

export const getAvailableClasses = async () => {
  const res = await axiosInstance('/classroom')

  return res.data
}
