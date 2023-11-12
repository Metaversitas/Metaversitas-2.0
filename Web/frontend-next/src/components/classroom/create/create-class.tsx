import React from 'react'
import { Card, Col, DatePicker, Form, Input, Row, Select, Typography } from 'antd'
import type { FormInstance } from 'antd'

type TCreateClass = {
  form: FormInstance
}

const options = [
  {
    label: 'Ganjil',
    value: 'odd'
  },
  {
    label: 'Genap',
    value: 'even'
  }
]
const CreateClass = ({ form }: TCreateClass) => {
  return (
    <Card>
      <Form layout={'vertical'} form={form} name={'class'}>
        <Form.Item label={'Judul Kelas'} name={'class_name'} rules={[{ required: true }]}>
          <Input size={'large'} />
        </Form.Item>
        <Row gutter={[8, 8]}>
          <Col xs={24} md={6}>
            <Form.Item label={'Jenis Kelas'} name={'subject_id'} rules={[{ required: true }]}>
              <Select size={'large'} options={options} />
            </Form.Item>
          </Col>
          <Col xs={24} md={6}>
            <Form.Item label={'Modul'} name={'secondary_subject_id'} rules={[{ required: true }]}>
              <Select size={'large'} options={options} />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item label={'Tahun Ajaran'} style={{ marginBottom: 0 }}>
          <Row gutter={[8, 8]} style={{ alignItems: 'baseline' }}>
            <Col xs={24} md={6}>
              <Form.Item name={'semester'} rules={[{ required: true }]}>
                <Select size={'large'} options={options} />
              </Form.Item>
            </Col>
            <Col xs={24} md={6}>
              <Form.Item name={'year_start'} rules={[{ required: true }]}>
                <DatePicker picker="year" size={'large'} style={{ width: '100%' }} />
              </Form.Item>
            </Col>
            <Col>
              <Typography.Title>/</Typography.Title>
            </Col>
            <Col xs={24} md={6}>
              <Form.Item name={'year_end'} rules={[{ required: true }]}>
                <DatePicker picker="year" size={'large'} style={{ width: '100%' }} />
              </Form.Item>
            </Col>
          </Row>
        </Form.Item>
        <Form.Item label={'Dosen Pengajar'} name={'teachers'}>
          <Input disabled bordered={false} />
        </Form.Item>
      </Form>
    </Card>
  )
}

export default CreateClass
