import React from 'react'
import { Flex } from 'antd'
import Text from '@/components/typography/text'

export type TProgram = {
  title: string
  description: string
}
const Program = ({ title, description }: TProgram) => {
  return (
    <Flex vertical gap={16}>
      <div style={{ height: 200, background: '#f0f0f0' }} />
      <Flex vertical gap={8}>
        <Text style={{ fontSize: 24, fontWeight: 600, display: 'block', textAlign: 'center' }}>
          {title}
        </Text>
        <Text style={{ fontSize: 16, display: 'block', textAlign: 'center' }}>{description}</Text>
      </Flex>
    </Flex>
  )
}

export default Program
