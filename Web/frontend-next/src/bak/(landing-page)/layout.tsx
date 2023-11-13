'use client'
import React, { useEffect, useState } from 'react'
import { Button, Col, Layout, Menu, Row } from 'antd'
import { usePathname } from 'next/navigation'
import Image from 'next/image'
import Link from 'next/link'
import Container from '@/components/container'
import LandingPageFooter from '@/components/landing-page/landing-page-footer'
import Content from '@/components/layout/content'
import Header from '@/components/layout/header'
import { navLandingPage } from '@/lib/layout/navigation-data'

const LandingPageLayout = ({ children }: React.PropsWithChildren) => {
  const pathname = usePathname()
  const [selectedKeys, setSelectedKeys] = useState<string>(pathname || '/')

  useEffect(() => {
    setSelectedKeys(pathname)
  }, [pathname])

  return (
    <Layout>
      <Header style={{ lineHeight: '100%' }}>
        <Container>
          <Row
            justify={'space-between'}
            align={'middle'}
            gutter={[24, 12]}
            style={{ height: '100%' }}
          >
            <Col>
              <Row align={'middle'} gutter={[8, 8]}>
                <Col>
                  <Image
                    src={'/image/metaversitas.png'}
                    alt={'Metaversitas'}
                    height={40}
                    width={57}
                  />
                </Col>
                <Col>
                  <Image
                    src={'/image/metaversitas-text.png'}
                    alt={'Metaversitas'}
                    height={23}
                    width={218}
                  />
                </Col>
              </Row>
            </Col>
            <Col>
              <Row align={'middle'} gutter={[24, 8]}>
                <Col>
                  <Image
                    src={'/image/tut-wuri-handayani.png'}
                    alt={'Tut Wuri Handayani'}
                    height={50}
                    width={50}
                  />
                </Col>
                <Col>
                  <Image
                    src={'/image/kampus-merdeka.png'}
                    alt={'Kampus Merdeka'}
                    height={45}
                    width={85}
                  />
                </Col>
              </Row>
            </Col>
          </Row>
        </Container>
      </Header>
      <Header style={{ backgroundColor: '#F8F8F8' }}>
        <Container>
          <Row align={'middle'} justify={'space-between'} style={{ width: '100%' }}>
            <Col>
              <Menu
                mode="horizontal"
                defaultSelectedKeys={[selectedKeys]}
                items={navLandingPage}
                style={{ backgroundColor: 'transparent' }}
                selectedKeys={[selectedKeys]}
              />
            </Col>
            <Col>
              <Row gutter={16}>
                <Col>
                  <Button>Daftar</Button>
                </Col>
                <Col>
                  <Link href={'/login'}>
                    <Button type={'primary'}>Masuk</Button>
                  </Link>
                </Col>
              </Row>
            </Col>
          </Row>
        </Container>
      </Header>
      <Content style={{ padding: '0 50px' }}>
        <Container>{children}</Container>
      </Content>
      <LandingPageFooter />
    </Layout>
  )
}

export default LandingPageLayout
