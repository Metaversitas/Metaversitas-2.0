import React from 'react'
import { Col, Row } from 'antd'
import Image from 'next/image'
import Widget from '../../../public/image/Asset 4@4x.png'
import Title from '@/components/typography/title'
import Text from '@/components/typography/text'
import LandingPageContent from '@/components/landing-page/landing-page-content'

const Installation = () => {
  return (
    <LandingPageContent id={'installation'}>
      <Row gutter={[56, 24]} justify={'center'} align={'middle'}>
        <Col md={12} lg={8}>
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
        <Col md={12} lg={10}>
          <Title level={4}>Download dan Instalasi</Title>
          <ol>
            <li>
              <Text>Unduh aplikasi dengan menyesuaikan OS perangkat anda.</Text>
            </li>
            <li>
              <Text>Instal aplikasi Metaversitas.exe yang telah diunduh.</Text>
            </li>
            <li>
              <Text>Setelah penginstalan selesai, aplikasi dapat langsung digunakan.</Text>
            </li>
          </ol>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default Installation
