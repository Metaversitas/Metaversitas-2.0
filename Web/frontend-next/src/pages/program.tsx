import React, { ReactElement } from 'react'
import Programs from '@/components/programs/programs'
import allPrograms from '@/lib/content/all-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import LandingPageLayout from '@/components/layout/landing-page'

const Program = () => {
  return (
    <>
      <Programs programs={allPrograms.programs} />
      <WidgetMetaversitas />
    </>
  )
}

Program.getLayout = function getLayout(page: ReactElement) {
  return <LandingPageLayout>{page}</LandingPageLayout>
}
export default Program
