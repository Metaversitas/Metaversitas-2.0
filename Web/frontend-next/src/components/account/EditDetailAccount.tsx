import React from 'react'
import { Button, Card, Col, Divider, Flex, Form, Input, Row, Typography } from 'antd'
import Link from 'next/link'
import { useRouter } from 'next/router'
import AccountItem from '@/components/account/AccountItem'

type TEditDetailAccount = {
  name: string
  email: string
  nim: string
  college: string
  study: string
}

const EditDetailAccount = ({ name, email, nim, college, study }: TEditDetailAccount) => {
  const [form] = Form.useForm()
  const router = useRouter()
  const onFinish = () => {
    form.validateFields().then(() => {
      router.push('/dashboard/account')
    })
  }
  return (
    <Card>
      <Flex vertical gap={16}>
        <Row justify={'space-between'}>
          <Col></Col>
          <Col>
            <Typography.Text>Detail Akun</Typography.Text>
          </Col>
          <Col>
            <Button type={'link'} onClick={onFinish} htmlType={'submit'}>
              Selesai
            </Button>
          </Col>
        </Row>
        <AccountItem title={'Name'} value={name} />
        <Form
          layout={'vertical'}
          colon={false}
          form={form}
          initialValues={{ email }}
          requiredMark={false}
        >
          <Form.Item
            label={'Email'}
            rules={[{ required: true, type: 'email' }]}
            style={{ marginBottom: 0 }}
            name={'email'}
          >
            <Input size={'large'} />
          </Form.Item>
        </Form>
        <AccountItem title={'NPM/NIM'} value={nim} />
        <AccountItem title={'Perguruan tinggi'} value={college} />
        <AccountItem title={'Program studi'} value={study} />
        <Divider style={{ margin: '8px 0' }} />
        <AccountItem title={'Kata sandi'} value={'. . . .'} />
        <Link href={'/dashboard/account/edit/password'}>Ubah kata sandi</Link>
      </Flex>
    </Card>
  )
}

export default EditDetailAccount
