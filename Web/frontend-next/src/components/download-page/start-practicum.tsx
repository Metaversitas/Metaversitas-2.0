import React from 'react'
import { Col, Row } from 'antd'
import Title from '@/components/typography/title'
import Text from '@/components/typography/text'
import LandingPageContent from '@/components/landing-page/landing-page-content'

const StartPracticum = () => {
  return (
    <LandingPageContent id={'start-practicum'}>
      <Row gutter={56} justify={'center'}>
        <Col md={12} lg={10}>
          <Title level={4}>Memulai Praktikum</Title>
          <ol>
            <li>
              <Text>
                Masuk ke akun anda pada website Metaversitas atau daftar disini jika belum mempunyai
                akun.
              </Text>
            </li>
            <li>
              <Text>
                Masuk ke halaman Kelas dan carilah kelas yang sesuai dengan mata praktikum anda.
              </Text>
            </li>
            <li>
              <Text> Enrol kelas yang telah dipilih.</Text>
            </li>
            <li>
              <Text>
                Setiap kelas akan secara otomatis tampil pada aplikasi Metaversitas yang telah masuk
                ke akun anda.
              </Text>
            </li>
            <li>
              <Text>Praktikum dapat dimulai jika pertemuan kelas sedang dilaksanakan.</Text>
            </li>
          </ol>
        </Col>
        <Col md={12} lg={8}>
          <div className={'demo-image'}></div>
        </Col>
      </Row>
    </LandingPageContent>
  )
}

export default StartPracticum
