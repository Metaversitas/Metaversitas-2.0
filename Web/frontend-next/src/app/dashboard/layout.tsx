import React, { PropsWithChildren } from 'react'
import { Flex, Layout, Menu } from 'antd'
import { headers } from 'next/headers'
import Sider from '@/components/layout/sider'
import Header from '@/components/layout/header'
import Content from '@/components/layout/content'
import Metaversitas from '@/components/icons/metaversitas'
import { navHomePage } from '@/lib/layout/navigation-data'

const App = ({ children }: PropsWithChildren) => {
  const headersList = headers()
  const defaultSelectedKeys = headersList.get('x-invoke-path') || '/'
  return (
    <Layout hasSider style={{ height: '100%' }}>
      <Sider breakpoint="lg" collapsedWidth="0" width={280}>
        <Flex gap={16} vertical align={'center'} style={{ padding: '55px 15px' }}>
          <Metaversitas />
          <Menu
            mode="inline"
            defaultSelectedKeys={[defaultSelectedKeys]}
            style={{ backgroundColor: 'transparent', border: 'none' }}
            items={navHomePage}
            id={'menu-dashboard'}
          />
        </Flex>
      </Sider>
      <Layout style={{ height: '100%' }}>
        <Header style={{ padding: 0 }} />
        <Content style={{ margin: '24px 16px 0' }}>
          <div style={{ padding: 24, minHeight: 360 }}>{children}</div>
        </Content>
      </Layout>
    </Layout>
  )
}

export default App
