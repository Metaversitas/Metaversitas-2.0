import React, { cloneElement, ReactElement } from 'react'
import { Col, Row, Space } from 'antd'
import Image from 'next/image'
import Text from '@/components/typography/text'
import Footer from '@/components/layout/footer'
import Content from '@/components/layout/content'
import LandingPageContent from '@/components/landing-page/landing-page-content'
import { navLandingPage } from '@/lib/layout/navigation-data'

const LandingPageFooter = () => {
  return (
    <Footer>
      <Content>
        <LandingPageContent>
          <Row gutter={[56, 56]} align={'stretch'} justify={{ md: 'start', lg: 'center' }}>
            <Col xs={24} sm={24} md={24} lg={8}>
              <Row gutter={[0, 24]}>
                <Col span={24}>
                  <Row
                    align={'middle'}
                    justify={{ md: 'start', lg: 'end' }}
                    gutter={8}
                    wrap={false}
                  >
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
                <Col span={24}>
                  <Row align={'middle'} justify={{ md: 'start', lg: 'end' }}>
                    <Col>
                      <Image
                        src={'/image/tut-wuri-handayani.png'}
                        alt={'Tut Wuri Handayani'}
                        height={50}
                        width={50}
                      />
                    </Col>
                  </Row>
                </Col>

                <Col span={24}>
                  <Row align={'middle'} justify={{ md: 'start', lg: 'end' }}>
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
            </Col>
            <Col xs={24} sm={24} md={24} flex={'none'}>
              <Space direction={'vertical'} size={24}>
                <Text strong>Navigation</Text>
                <Space direction={'vertical'} size={16}>
                  {navLandingPage.map((nav) => (
                    <div key={nav.key}>
                      {cloneElement(nav.label as ReactElement, { style: { color: '#000' } })}
                    </div>
                  ))}
                </Space>
              </Space>
            </Col>
            <Col xs={24} sm={24} md={24} lg={10}>
              <Space direction={'vertical'} size={24}>
                <Text strong>
                  Direktorat Jenderal Pendidikan Tinggi, Riset, dan Teknologi (Diktiristek)
                </Text>
                <Text>Jalan Jenderal Sudirman, Jakarta Pusat 10270</Text>
                <Text>
                  +62xxxxxxxxxxx <br />
                  www.websitename.co.id <br />
                  Other contact (optional)
                </Text>
                <Text>
                  Hak Cipta 2023 Kementerian Pendidikan dan Kebudayaan RI <br />
                  Foto oleh ITS, ITERA, and UI
                </Text>
              </Space>
            </Col>
          </Row>
        </LandingPageContent>
      </Content>
    </Footer>
  )
}

export default LandingPageFooter
