import React from 'react'
import { Avatar, Card, Flex, Typography } from 'antd'
import { UserOutlined } from '@ant-design/icons'

type TProfilePicture = {
  name: string
}
const ProfilePicture = ({ name }: TProfilePicture) => {
  return (
    <Card>
      <Flex vertical gap={20} align={'center'}>
        <Typography.Text>Foto Profil</Typography.Text>
        <Avatar size={140} icon={<UserOutlined />} />
        <Typography.Title level={3}>{name}</Typography.Title>
      </Flex>
    </Card>
  )
}

export default ProfilePicture
