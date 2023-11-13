import React from 'react'
import Programs from '@/components/programs/programs'
import allPrograms from '@/lib/content/all-programs'
import WidgetMetaversitas from '@/components/widget-metaversitas/widget-metaversitas'
import ThemeProvider from '@/theme/theme-provider'

const Program = () => {
  return (
    <ThemeProvider>
      <Programs programs={allPrograms.programs} />
      <WidgetMetaversitas />
    </ThemeProvider>
  )
}

export default Program
