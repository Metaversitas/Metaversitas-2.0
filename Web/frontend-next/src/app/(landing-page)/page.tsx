import React from 'react'
import LandingPageHero from '@/components/landing-page/landing-page-hero'
import LandingPagePrograms from '@/components/landing-page/landing-page-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'

const Home = () => {
  return (
    <>
      <LandingPageHero />
      <LandingPagePrograms />
      <WidgetMetaversitas />
    </>
  )
}

export default Home
