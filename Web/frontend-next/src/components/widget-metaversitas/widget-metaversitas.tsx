import React from 'react'
import { Button, Col, Row, Space } from 'antd'
import Link from 'next/link'
import Title from '@/components/typography/title'
import Text from '@/components/typography/text'
import LandingPageContent from '@/components/landing-page/landing-page-content'

const WidgetMetaversitas = () => {
  return (
    <LandingPageContent>
      <Row align={'middle'} gutter={56} justify={'center'}>
        <Col md={24} lg={10}>
          <div style={{ background: '#f0f0f0', height: 330 }} />
        </Col>
        <Col md={24} lg={10}>
          <Space size={24} direction={'vertical'}>
            <Space size={8} direction={'vertical'}>
              <Title level={4}>Metaversitas</Title>
              <Text>Unduh aplikasi METAVERSITAS untuk melakukan praktikum virtual.</Text>
              <Link href={'/download#installation'} style={{ color: '#0033B7' }}>
                Petunjuk penggunaan
              </Link>
            </Space>
            <Row gutter={[16, 16]}>
              <Col>
                <Button type={'primary'}>Unduh untuk Windows</Button>
              </Col>
              <Col>
                <Button type={'primary'}>Unduh untuk IOS</Button>
              </Col>
            </Row>
          </Space>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default WidgetMetaversitas
