import React from 'react'
import { Col, Flex, Row } from 'antd'
import Image from 'next/image'
import HeroImage from '../../../public/image/Asset 2_1@4x.png'
import LandingPageContent from '@/components/landing-page/landing-page-content'
import Text from '@/components/typography/text'

const LandingPageHero = () => {
  return (
    <LandingPageContent>
      <Row gutter={[24, 24]} justify={'space-between'}>
        <Col xs={{ order: 2, span: 24 }} md={24} lg={{ order: 1, span: 12 }}>
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
        <Col xs={{ order: 1, span: 24 }} md={24} lg={{ order: 2, span: 12 }} flex={'auto'}>
          <Flex justify={'center'}>
            <Image
              src={HeroImage}
              alt={'hero'}
              style={{
                maxWidth: 587,
                width: '100%',
                height: 'auto'
              }}
            />
          </Flex>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default LandingPageHero
