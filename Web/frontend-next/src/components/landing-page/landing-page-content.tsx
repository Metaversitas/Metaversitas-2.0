import React, { PropsWithChildren } from 'react'

type TLandingPageContent = {
  id?: string
} & PropsWithChildren

const LandingPageContent = ({ children, id }: TLandingPageContent) => {
  return (
    <div style={{ padding: '80px 0' }} id={id}>
      {children}
    </div>
  )
}

export default LandingPageContent
