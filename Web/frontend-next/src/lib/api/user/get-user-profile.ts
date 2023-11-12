import axiosInstance from '@/lib/axios-instance'

export interface UserProfileResponse {
  data: Data
  status: boolean
}

export interface Data {
  faculty_id: number
  faculty_name: string
  full_name: string
  gender: string
  in_game_nickname: string
  university_name: string
  user_id: string
  user_univ_role: string
  user_university_id: number
}

export const getUserProfile = (): Promise<Response> => {
  return axiosInstance.get('/user/profile')
}
