import React, { ReactElement } from 'react'
import About from '@/components/about/about'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import LandingPageLayout from '@/components/layout/landing-page'

const AboutPage = () => {
  return (
    <>
      <About />
      <WidgetMetaversitas />
    </>
  )
}

AboutPage.getLayout = function getLayout(page: ReactElement) {
  return <LandingPageLayout>{page}</LandingPageLayout>
}
export default AboutPage
