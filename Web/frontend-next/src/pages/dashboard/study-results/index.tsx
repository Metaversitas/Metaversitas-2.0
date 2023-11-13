import React, { ReactElement } from 'react'
import LayoutDashboard from '@/components/layout/dashboard'

const Index = () => {
  return <></>
}

Index.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}
export default Index
