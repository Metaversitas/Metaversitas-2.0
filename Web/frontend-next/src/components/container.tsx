import React, { PropsWithChildren } from 'react'

const Container = ({ children }: PropsWithChildren) => {
  return (
    <section style={{ width: '100%', maxWidth: 1440, margin: '0 auto', height: '100%' }}>
      {children}
    </section>
  )
}

export default Container
