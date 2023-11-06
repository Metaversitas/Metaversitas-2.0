import React from 'react'
import { Badge, Card, Col, Row } from 'antd'
import Link from 'next/link'
import Text from '@/components/typography/text'
import Title from '@/components/typography/title'

type TClass = {
  subject: string
  topic: string
  lecturer: string
  classId: string
  isEnroll: boolean
}
const Class = ({ subject, topic, lecturer, classId, isEnroll }: TClass) => {
  return (
    <Card>
      <Row gutter={[0, 16]}>
        <Col span={24}>
          <Badge count={subject} style={{ backgroundColor: '#DCDDF7', color: '#6B65FB' }} />
        </Col>
        <Col span={24}>
          <Title level={5}>{topic}</Title>
        </Col>
        <Col span={24}>
          <Row align={'bottom'} justify={'space-between'}>
            <Col>
              <Text style={{ fontSize: 14 }} strong>
                {lecturer}
              </Text>
              <Text type={'secondary'} style={{ fontSize: 14 }}>
                PTA 2022/2023
              </Text>
            </Col>
            <Col>
              <Link href={`/classroom/${classId}`}>{isEnroll ? 'Enroll' : 'Lihat'}</Link>
            </Col>
          </Row>
        </Col>
      </Row>
    </Card>
  )
}

export default Class
