import React from 'react'
import { Flex, Typography } from 'antd'

type TAccountItem = {
  title: string
  value: string
}
const AccountItem = ({ title, value }: TAccountItem) => {
  return (
    <Flex vertical gap={8}>
      <Typography.Title level={5}>{title}</Typography.Title>
      <Typography.Text style={{ color: '#868686' }}>{value}</Typography.Text>
    </Flex>
  )
}

export default AccountItem
