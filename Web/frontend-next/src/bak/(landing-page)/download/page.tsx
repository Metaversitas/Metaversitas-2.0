'use client'
import React from 'react'
import Download from '@/components/download-page/download'
import Installation from '@/components/download-page/installation'
import StartPracticum from '@/components/download-page/start-practicum'
import ThemeProvider from '@/theme/theme-provider'

const Page = () => {
  return (
    <ThemeProvider>
      <Download />
      <Installation />
      <StartPracticum />
    </ThemeProvider>
  )
}

export default Page
