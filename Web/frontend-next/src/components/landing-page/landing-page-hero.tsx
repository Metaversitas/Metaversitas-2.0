import React from 'react'
import { Col, Row } from 'antd'
import LandingPageContent from '@/components/landing-page/landing-page-content'
import Text from '@/components/typography/text'

const LandingPageHero = () => {
  return (
    <LandingPageContent>
      <Row gutter={24} justify={'space-between'}>
        <Col md={24} lg={12}>
          <Row align={'middle'} style={{ height: '100%' }}>
            <Col flex={'auto'}>
              <Text style={{ fontSize: 32, fontWeight: 700 }}>
                Belajar dengan cara yang lebih menyenangkan
              </Text>
              <br />
              <Text style={{ fontSize: 20 }}>
                Metaversitas adalah platform pembelajaran virtual berbasis Metaverse yang
                menghadirkan pengalaman pembelajaran yang seru dan interaktif bagi mahasiswa dan
                dosen. Dengan Metaversitas, pengguna dapat menjelajahi dunia praktikum secara
                virtual, menjalankan eksperimen, dan berinteraksi dengan materi pembelajaran secara
                mendalam, menjadikan pembelajaran lebih menarik dan efektif.
              </Text>
            </Col>
          </Row>
        </Col>
        <Col md={24} lg={12} flex={'auto'}>
          <div style={{ height: 700, background: '#f0f0f0' }} />
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default LandingPageHero
