import React from 'react'
import { Col, Row } from 'antd'
import Title from '@/components/typography/title'
import Text from '@/components/typography/text'
import LandingPageContent from '@/components/landing-page/landing-page-content'

const Installation = () => {
  return (
    <LandingPageContent id={'installation'}>
      <Row gutter={56} justify={'center'}>
        <Col md={12} lg={8}>
          <div className={'demo-image'}></div>
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
