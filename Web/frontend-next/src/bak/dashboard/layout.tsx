import React, { PropsWithChildren } from 'react'
import { Flex, Layout, Menu } from 'antd'
import { headers } from 'next/headers'
import Sider from '@/components/layout/sider'
import Content from '@/components/layout/content'
import Metaversitas from '@/components/icons/metaversitas'
import { navHomePage } from '@/lib/layout/navigation-data'
import DashboardHeader from '@/components/layout/dashboard-header'
import ThemeProvider from '@/theme/theme-provider'

const App = ({ children }: PropsWithChildren) => {
  const headersList = headers()
  const defaultSelectedKeys = headersList.get('x-invoke-path') || '/'
  return (
    <ThemeProvider>
      <Layout hasSider>
        <Sider breakpoint="lg" collapsedWidth="0" width={280}>
          <Flex gap={16} vertical align={'center'} style={{ padding: '55px 15px' }}>
            <Metaversitas />
            <Menu
              mode="inline"
              defaultSelectedKeys={[defaultSelectedKeys]}
              style={{ backgroundColor: 'inherit', border: 'none' }}
              items={navHomePage}
              id={'menu-dashboard'}
            />
          </Flex>
        </Sider>
        <Layout style={{ minHeight: '100vh' }}>
          <DashboardHeader />
          <Content style={{ padding: 50 }}>{children}</Content>
          {/*{children}*/}
        </Layout>
      </Layout>
    </ThemeProvider>
  )
}

export default App
