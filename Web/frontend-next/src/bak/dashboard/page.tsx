import React from 'react'
import { Flex } from 'antd'
import UpcomingClasses from '@/components/dashboard/upcoming-classes'
import AvailableClasses from '@/components/dashboard/available-classes'
import JoinedClasses from '@/components/dashboard/joined-classes'
import ThemeProvider from '@/theme/theme-provider'

const Page = () => {
  return (
    <ThemeProvider>
      <Flex gap={60} vertical>
        <UpcomingClasses />
        <AvailableClasses />
        <JoinedClasses />
      </Flex>
    </ThemeProvider>
  )
}

// export async function

export default Page
