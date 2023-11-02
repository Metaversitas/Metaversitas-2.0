import React from 'react'
import { Col, Row } from 'antd'
import Title from '@/components/typography/title'
import LandingPageContent from '@/components/landing-page/landing-page-content'
import Text from '@/components/typography/text'

const About = () => {
  return (
    <LandingPageContent>
      <Row gutter={[0, 56]} justify={'center'}>
        <Col md={24} lg={24}>
          <Title level={4} style={{ textAlign: 'center' }}>
            Tentang Metaversitas
          </Title>
        </Col>
        <Col md={24} lg={20}>
          <Text style={{ textAlign: 'center' }}>
            Metaversitas adalah aplikasi inovatif yang disusun oleh Kementerian Pendidikan,
            Kebudayaan, Riset, dan Teknologi (Kemendikbudristek), dirancang untuk memberikan
            pengalaman praktikum yang mendalam dan mendidik bagi mahasiswa dan dosen secara virtual.
            Aplikasi ini menjadikan pengguna dapat menjelajahi lingkungan praktikum yang imersif,
            menjalani eksperimen, dan berinteraksi dengan sumber daya serta simulasi yang relevan,
            tanpa harus berada di lokasi fisik praktikum. Metaversitas membuka pintu bagi
            pembelajaran berbasis teknologi yang lebih dinamis, memungkinkan para peserta untuk
            merasakan pengalaman praktikum dengan cara yang inovatif dan efisien.
          </Text>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default About
