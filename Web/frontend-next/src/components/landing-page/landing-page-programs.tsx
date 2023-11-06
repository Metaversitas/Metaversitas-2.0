import React from 'react'
import Programs from '@/components/programs/programs'
import allPrograms from '@/lib/content/all-programs'

const LandingPagePrograms = () => {
  return <Programs programs={allPrograms.programs.slice(0, 3)} />
}

export default LandingPagePrograms
