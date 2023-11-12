import React from 'react'
import About from '@/components/about/about'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import ThemeProvider from '@/theme/theme-provider'

const Page = () => {
  return (
    <ThemeProvider>
      <About />
      <WidgetMetaversitas />
    </ThemeProvider>
  )
}

export default Page
