import React from 'react'
import { Button, Col, Row, Space } from 'antd'
import LandingPageContent from '@/components/landing-page/landing-page-content'
import Title from '@/components/typography/title'
import Text from '@/components/typography/text'

const Download = () => {
  return (
    <LandingPageContent>
      <Row align={'middle'} gutter={56} justify={'center'}>
        <Col md={24} lg={10}>
          <Row justify={'center'}>
            <Space size={56} direction={'vertical'}>
              <Title level={4} style={{ textAlign: 'center' }}>
                Metaversitas
              </Title>
              <Space size={24} direction={'vertical'}>
                <Text style={{ textAlign: 'center' }}>
                  Unduh aplikasi METAVERSITAS untuk melakukan praktikum virtual.
                </Text>
                <Row gutter={16} justify={'center'}>
                  <Col>
                    <Button type={'primary'}>Unduh untuk Windows</Button>
                  </Col>
                  <Col>
                    <Button type={'primary'}>Unduh untuk IOS</Button>
                  </Col>
                </Row>
              </Space>
              <Text style={{ textAlign: 'center' }}>Lihat petunjuk penggunaan dibawah</Text>
            </Space>
          </Row>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default Download
