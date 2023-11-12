import React, { ReactElement, useState } from 'react'
import type { CollapseProps } from 'antd'
import { Button, Flex, Form, Input, Modal, Typography } from 'antd'
import { PlusOutlined } from '@ant-design/icons'
import { useRouter } from 'next/router'
import LayoutDashboard from '@/components/layout/dashboard'
import Collapse from '@/components/collapse/collapse'

const text = `
  A dog is a type of domesticated animal.
  Known for its loyalty and faithfulness,
  it can be found as a welcome guest in many households across the world.
`

const items: CollapseProps['items'] = [
  {
    key: '1',
    label: 'This is panel header with arrow icon',
    children: <p>{text}</p>
  },
  {
    key: '2',
    label: 'This is panel header with no arrow icon'
  }
]

const Index = () => {
  const [isModalOpen, setIsModalOpen] = useState(false)
  const router = useRouter()
  const showModal = () => {
    setIsModalOpen(true)
  }

  const handleOk = () => {
    setIsModalOpen(false)
  }

  const handleCancel = () => {
    setIsModalOpen(false)
  }

  const onFinish = () => {
    handleOk()
  }
  return (
    <Flex vertical gap={16}>
      <Flex justify={'space-between'}>
        <Typography.Text>Kategori Soal</Typography.Text>
        <Button type={'primary'} size={'large'} icon={<PlusOutlined />} onClick={showModal}>
          Buat Kategori
        </Button>
      </Flex>
      <Collapse
        items={items}
        expandIconPosition={'end'}
        collapsible={'icon'}
        onChange={() => router.push('/dashboard/')}
      ></Collapse>
      <Modal
        closable={false}
        open={isModalOpen}
        onOk={handleOk}
        onCancel={handleCancel}
        footer={false}
        destroyOnClose
      >
        <Flex vertical>
          <Flex vertical gap={24}>
            <Flex vertical gap={16} align={'center'}>
              <Typography.Title>Nama Kategori</Typography.Title>
              <Typography.Text>Description</Typography.Text>
            </Flex>
            <Flex vertical gap={40}>
              <Form
                layout={'vertical'}
                requiredMark={false}
                colon={false}
                onFinish={onFinish}
                preserve={false}
              >
                <Form.Item name={'category'} rules={[{ required: true }]}>
                  <Input placeholder={'Nama kategori'} size={'large'} />
                </Form.Item>
                <Flex gap={40} justify={'center'}>
                  <Button type={'text'} htmlType={'button'} onClick={handleCancel}>
                    Batalkan
                  </Button>
                  <Button type={'primary'} htmlType={'submit'}>
                    Simpan
                  </Button>
                </Flex>
              </Form>
            </Flex>
          </Flex>
        </Flex>
      </Modal>
    </Flex>
  )
}
Index.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}
export default Index
