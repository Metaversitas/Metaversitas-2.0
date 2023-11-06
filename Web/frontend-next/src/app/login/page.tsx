'use client'
import Image from 'next/image'
import React, { Fragment } from 'react'
import { Card, Layout, Form, Input, Button, Row, Col, Divider } from 'antd'
import { useRouter } from 'next/navigation'
import BG from '../../../public/image.jpg'
import axiosInstance from '@/lib/axios-instance'
import styles from '@/styles.module.css'
import Title from '@/components/typography/title'
import Content from '@/components/layout/content'
import Header from '@/components/layout/header'

type Payload = {
  user: {
    email: string
    password: string
  }
}
const Page = () => {
  const router = useRouter()
  const [form] = Form.useForm()

  const onFinish = async (payload: Payload) => {
    try {
      await axiosInstance.post('/auth/login', payload, { withCredentials: true })
      router.push('/dashboard')
    } catch (error) {}
  }
  return (
    <Fragment>
      <div className={styles.bgWrap}>
        <Image
          src={BG}
          placeholder="blur"
          quality={100}
          fill
          sizes="100vw"
          style={{
            objectFit: 'cover'
          }}
          alt={'image'}
        />
      </div>
      <Layout
        style={{
          backgroundColor: 'transparent',
          maxWidth: 668,
          margin: '0 auto',
          padding: '48px 16px',
          height: '100%'
        }}
      >
        <Header style={{ backgroundColor: 'inherit', lineHeight: '100%' }}>
          <Row justify={'center'} align={'middle'} gutter={24}>
            <Col style={{ height: 'fit-content' }}>
              <Image src={'/image/kampus-merdeka.png'} alt={'Tut Wuri Handayani'} />
            </Col>
            <Col>
              <Image src={'/image/kampus-merdeka.png'} alt={'Kampus Merdeka'} />
            </Col>
            <Col>
              <Row align={'middle'} gutter={8}>
                <Col>
                  <Image src={'/image/metaversitas.png'} alt={'Metaversitas'} />
                </Col>
                <Col>
                  <Image src={'/image/metaversitas-text.png'} alt={'Metaversitas'} height={23} />
                </Col>
              </Row>
            </Col>
          </Row>
        </Header>
        <Content style={{ backgroundColor: 'inherit', padding: '48px 0', height: '100%' }}>
          <Card style={{ height: '100%' }}>
            <Form layout={'vertical'} form={form} onFinish={onFinish}>
              <Title level={3}>
                Masuk ke <br />
                Metaversitas
              </Title>
              <Form.Item label={'Email'} name={['user', 'email']}>
                <Input type={'email'} size={'large'} />
              </Form.Item>
              <Form.Item label={'Kata Sandi'} name={['user', 'password']}>
                <Input type={'text'} size={'large'} />
              </Form.Item>
              <Row justify={'center'}>
                <Col>
                  <Button htmlType={'submit'} size={'large'} type={'primary'}>
                    Masuk
                  </Button>
                </Col>
              </Row>
              <Row justify={'center'}>
                <Col span={20}>
                  <Divider>atau masuk melalui</Divider>
                </Col>
              </Row>
            </Form>
          </Card>
        </Content>
      </Layout>
    </Fragment>
  )
}

export default Page
