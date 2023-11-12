import React, { ReactElement } from 'react'
import Download from '@/components/download-page/download'
import Installation from '@/components/download-page/installation'
import StartPracticum from '@/components/download-page/start-practicum'
import LandingPageLayout from '@/components/layout/landing-page'

const DownloadPage = () => {
  return (
    <>
      <Download />
      <Installation />
      <StartPracticum />
    </>
  )
}

DownloadPage.getLayout = function getLayout(page: ReactElement) {
  return <LandingPageLayout>{page}</LandingPageLayout>
}
export default DownloadPage
