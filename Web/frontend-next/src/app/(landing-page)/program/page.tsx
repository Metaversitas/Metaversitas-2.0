import React from 'react'
import Programs from '@/components/programs/programs'
import allPrograms from '@/lib/content/all-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'

const Program = () => {
  return (
    <>
      <Programs programs={allPrograms.programs} />
      <WidgetMetaversitas />
    </>
  )
}

export default Program
