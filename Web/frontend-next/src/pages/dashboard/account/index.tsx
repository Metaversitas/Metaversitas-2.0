import React, { ReactElement } from 'react'
import { Col, Row } from 'antd'
import LayoutDashboard from '@/components/layout/dashboard'
import ProfilePicture from '@/components/account/ProfilePicture'
import DetailAccount from '@/components/account/DetailAccount'

const Index = () => {
  return (
    <Row gutter={[32, 32]}>
      <Col lg={10}>
        <ProfilePicture name={'A'} />
      </Col>
      <Col lg={14}>
        <DetailAccount name={'A'} college={'A'} email={'A'} nim={'0'} study={'A'} />
      </Col>
    </Row>
  )
}

Index.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}
export default Index
