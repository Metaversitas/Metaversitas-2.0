import React from 'react'
import ThemeProvider from '@/theme/theme-provider'

const Page = ({ params }: { params: { classId: string } }) => {
  return <ThemeProvider>detail class: {params.classId}</ThemeProvider>
}
export default Page
