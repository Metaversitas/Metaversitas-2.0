import axiosInstance from '@/lib/axios-instance'

export interface GetClassroomResponse {
  data: Datum[]
}

export interface Datum {
  capacity: number
  class_id: string
  class_name: string
  current_meeting_id: string
  description: string
  is_active: boolean
  is_enrolled: boolean
  meetings: Meeting[]
  secondary_subject: null
  semester: string
  subject_id: string
  subject_name: string
  teachers: Teacher[]
  year_end: string
  year_start: string
}

export interface Meeting {
  created_at: Date
  end_time: Date
  is_active: boolean
  meeting_id: string
  meeting_name: string
  meeting_number: number
  start_time: Date
  topic_description: string
  updated_at: Date
  description?: string
}

export interface Teacher {
  teacher_id: string
  teacher_name: string
}

export const getClassroom = async (): Promise<GetClassroomResponse> => {
  return await axiosInstance.get('/classroom')
}
