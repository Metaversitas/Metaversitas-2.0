import React from 'react'
import { Flex } from 'antd'
import UpcomingClasses from '@/components/dashboard/upcoming-classes'
import AvailableClasses from '@/components/dashboard/available-classes'
import JoinedClasses from '@/components/dashboard/joined-classes'

const Page = () => {
  return (
    <Flex gap={60} vertical>
      <UpcomingClasses />
      <AvailableClasses />
      <JoinedClasses />
    </Flex>
  )
}

export default Page
