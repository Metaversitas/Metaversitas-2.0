import React from 'react'
import { Button, Card, Col, DatePicker, Flex, Form, Input, Row, TimePicker } from 'antd'
import type { FormListFieldData } from 'antd'

const format = 'HH:mm'

type TFormList = {
  field: FormListFieldData
}

type THeaderMeeting = {
  remove: (index: number | number[]) => void
} & TFormList

export const HeaderMeeting = ({ field, remove }: THeaderMeeting) => {
  return (
    <Flex justify={'space-between'} align={'center'}>
      <Form.Item
        label="Judul Pertemuan"
        name={[field.name, 'meeting_name']}
        rules={[{ required: true }]}
      >
        <Input size={'large'} />
      </Form.Item>
      <Button
        onClick={() => {
          remove(field.name)
        }}
        type={'primary'}
        htmlType={'button'}
      >
        Hapus
      </Button>
    </Flex>
  )
}
const Meeting = ({ field }: TFormList) => {
  return (
    <Card>
      <Form.Item
        label={'Topik Pertemuan'}
        rules={[{ required: true }]}
        name={[field.name, 'topic_description']}
      >
        <Input />
      </Form.Item>
      <Form.Item
        label={'Deskripsi'}
        rules={[{ required: true }]}
        name={[field.name, 'description']}
      >
        <Input />
      </Form.Item>
      <Form.Item label={'Jadwal Kelas'} style={{ marginBottom: 0 }}>
        <Row gutter={[8, 8]} style={{ alignItems: 'baseline' }}>
          <Col xs={24} md={6}>
            <Form.Item name={[field.name, 'semester']} rules={[{ required: true }]}>
              <DatePicker picker="year" size={'large'} style={{ width: '100%' }} />
            </Form.Item>
          </Col>
          <Col xs={24} md={6}>
            <Form.Item name={[field.name, 'start_time']} rules={[{ required: true }]}>
              <TimePicker size={'large'} style={{ width: '100%' }} format={format} />
            </Form.Item>
          </Col>
          <Col xs={24} md={6}>
            <Form.Item name={[field.name, 'end_time']} rules={[{ required: true }]}>
              <TimePicker size={'large'} style={{ width: '100%' }} format={format} />
            </Form.Item>
          </Col>
        </Row>
      </Form.Item>
    </Card>
  )
}

export default Meeting
