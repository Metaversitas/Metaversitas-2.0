import React from 'react'
import { Card, Col, Divider, Flex, Row, Typography } from 'antd'
import Link from 'next/link'
import AccountItem from '@/components/account/AccountItem'

type TDetailAccount = {
  name: string
  email: string
  nim: string
  college: string
  study: string
}

const DetailAccount = ({ name, email, nim, college, study }: TDetailAccount) => {
  return (
    <Card>
      <Flex vertical gap={16}>
        <Row justify={'space-between'}>
          <Col></Col>
          <Col>
            <Typography.Text>Detail Akun</Typography.Text>
          </Col>
          <Col>
            <Link href={'/dashboard/account/edit'}>Ubah</Link>
          </Col>
        </Row>
        <AccountItem title={'Name'} value={name} />
        <AccountItem title={'Email'} value={email} />
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

export default DetailAccount
