import React from 'react'
import LandingPageHero from '@/components/landing-page/landing-page-hero'
import LandingPagePrograms from '@/components/landing-page/landing-page-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import ThemeProvider from '@/theme/theme-provider'

const Home = () => {
  return (
    <ThemeProvider>
      <LandingPageHero />
      <LandingPagePrograms />
      <WidgetMetaversitas />
    </ThemeProvider>
  )
}

export default Home
