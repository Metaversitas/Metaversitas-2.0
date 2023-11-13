import React, { PropsWithChildren } from 'react'
import { Flex, Layout, Menu } from 'antd'
import { useRouter } from 'next/router'
import Sider from '@/components/layout/sider'
import Content from '@/components/layout/content'
import Metaversitas from '@/components/icons/metaversitas'
import { navHomePage } from '@/lib/layout/navigation-data'
import DashboardHeader from '@/components/layout/dashboard-header'

const LayoutDashboard = ({ children }: PropsWithChildren) => {
  const router = useRouter()
  const defaultSelectedKeys = router.pathname ?? '/dashboard'
  return (
    <>
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
    </>
  )
}

export default LayoutDashboard
