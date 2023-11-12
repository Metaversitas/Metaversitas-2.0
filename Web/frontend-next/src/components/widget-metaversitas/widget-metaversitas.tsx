import React from 'react'
import { Button, Col, Row, Space, Typography } from 'antd'
import Link from 'next/link'
import Image from 'next/image'
import Widget from '../../../public/image/Asset 4@4x.png'
import LandingPageContent from '@/components/landing-page/landing-page-content'

const { Title, Text } = Typography
const WidgetMetaversitas = () => {
  return (
    <LandingPageContent>
      <Row align={'middle'} gutter={[56, 24]} justify={'center'}>
        <Col md={24} lg={10}>
          <Row justify={{ md: 'center', lg: 'end' }}>
            <Col>
              <Image
                src={Widget}
                alt={'widget'}
                style={{
                  maxWidth: 380,
                  width: '100%',
                  height: 'auto'
                }}
              />
            </Col>
          </Row>
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
