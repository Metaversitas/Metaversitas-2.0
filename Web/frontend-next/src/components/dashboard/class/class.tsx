import React from 'react'
import { Badge, Button, Card, Col, Row, Typography } from 'antd'
import Link from 'next/link'

const { Title, Text } = Typography

type TClass = {
  subject: string
  topic: string
  lecturer: string
  classId: string
  isEnroll: boolean
  pta: string
}
const Class = ({ subject, topic, lecturer, classId, isEnroll, pta }: TClass) => {
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
                Dosen: {lecturer}
              </Text>
              <Text type={'secondary'} style={{ fontSize: 14 }}>
                PTA {pta}
              </Text>
            </Col>
            <Col>
              <Link
                href={`/dashboard/classroom/${classId}`}
                // href={{
                //   pathname: '/dashboard/classroom',
                //   query: { classId }
                // }}
              >
                <Button>{isEnroll ? 'Enroll' : 'Lihat'}</Button>
              </Link>
            </Col>
          </Row>
        </Col>
      </Row>
    </Card>
  )
}

export default Class
