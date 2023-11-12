import React, { ReactElement } from 'react'
import { Flex } from 'antd'
import UpcomingClasses from '@/components/dashboard/upcoming-classes'
import AvailableClasses from '@/components/dashboard/available-classes'
import JoinedClasses from '@/components/dashboard/joined-classes'
import LayoutDashboard from '@/components/layout/dashboard'

const Index = () => {
  return (
    <>
      <Flex gap={60} vertical>
        <UpcomingClasses />
        <AvailableClasses />
        <JoinedClasses />
      </Flex>
    </>
  )
}

Index.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}

export default Index
