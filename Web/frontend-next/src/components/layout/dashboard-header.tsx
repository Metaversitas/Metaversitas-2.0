import React, { ReactNode } from 'react'
import { Col, Dropdown, Flex, MenuProps, Row } from 'antd'
import Image from 'next/image'
import Header from '@/components/layout/header'
import Down from '@/components/icons/down'
import GoBack from '@/components/go-back'
import Text from '@/components/typography/text'

const items: MenuProps['items'] = [
  {
    label: 'Akun',
    key: 'Akun',
    icon: null
  },
  {
    label: 'Keluar',
    key: 'keluar',
    icon: null
  }
]

type TDashboradHeader = {
  children?: ReactNode
  withGoBack?: boolean
  pathname?: string
  description?: string
}

const DashboardHeader = ({ children, withGoBack, pathname, description }: TDashboradHeader) => {
  return (
    <Header>
      <Row justify={'space-between'} align={'middle'}>
        <Col>
          <Row>
            <Col>{withGoBack ? <GoBack /> : null}</Col>
            <Col>
              <Text>{pathname}</Text>
              <Text>{description}</Text>
            </Col>
          </Row>
        </Col>
        <Col>{children}</Col>
        <Col>
          <Dropdown menu={{ items }} trigger={['click']}>
            <Flex gap={16} align={'center'}>
              <Image
                src={'/image.jpg'}
                width={30}
                height={30}
                style={{ borderRadius: '50%' }}
                alt={'user'}
              />
              <Flex gap={8} align={'center'}>
                Inpo
                <Down />
              </Flex>
            </Flex>
          </Dropdown>
        </Col>
      </Row>
    </Header>
  )
}

export default DashboardHeader
