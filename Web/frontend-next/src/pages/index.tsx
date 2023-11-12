import React, { ReactElement } from 'react'
import LandingPageHero from '@/components/landing-page/landing-page-hero'
import LandingPagePrograms from '@/components/landing-page/landing-page-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import LandingPageLayout from '@/components/layout/landing-page'

const Home = () => {
  return (
    <>
      <LandingPageHero />
      <LandingPagePrograms />
      <WidgetMetaversitas />
    </>
  )
}

Home.getLayout = function getLayout(page: ReactElement) {
  return <LandingPageLayout>{page}</LandingPageLayout>
}

export default Home
