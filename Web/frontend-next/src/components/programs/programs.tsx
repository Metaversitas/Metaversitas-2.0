import React from 'react'
import { Col, Flex, Row } from 'antd'
import Program, { TProgram } from '@/components/programs/program'
import Title from '@/components/typography/title'
import LandingPageContent from '@/components/landing-page/landing-page-content'

export type TPrograms = {
  programs: TProgram[]
}
const Programs = ({ programs }: TPrograms) => {
  return (
    <LandingPageContent>
      <Flex vertical gap={64}>
        <Title level={4} style={{ textAlign: 'center' }}>
          Programs
        </Title>
        <Row gutter={[40, 40]}>
          {programs.length > 0
            ? programs.map((item) => (
                <Col key={item.title} sm={12} lg={8}>
                  <Program title={item.title} description={item.description} />
                </Col>
              ))
            : null}
        </Row>
      </Flex>
    </LandingPageContent>
  )
}

export default Programs
