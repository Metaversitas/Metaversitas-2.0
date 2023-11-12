import React, { ReactNode } from 'react'
import DashboardHeader from '@/components/layout/dashboard-header'
import Content from '@/components/layout/content'

const DashboardLayout = ({
  withGoBack,
  pathname,
  children
}: {
  withGoBack?: boolean
  pathname?: string
  children?: ReactNode
}) => {
  return (
    <>
      <DashboardHeader withGoBack={withGoBack} pathname={pathname} />

      <Content style={{ padding: '0 50px' }}>{children}</Content>
    </>
  )
}

export default DashboardLayout
