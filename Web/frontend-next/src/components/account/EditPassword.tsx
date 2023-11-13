import React from 'react'
import { Button, Card, Flex, Form, Input } from 'antd'
import Link from 'next/link'

const EditPassword = () => {
  return (
    <Form layout={'vertical'} requiredMark={false} colon={false}>
      <Flex vertical gap={24}>
        <Card>
          <Form.Item name={'password'} label={'Kata sandi lama'} rules={[{ required: true }]}>
            <Input.Password visibilityToggle={false} size={'large'} />
          </Form.Item>
          <Form.Item name={'newPassword'} label={'Kata sandi baru'} rules={[{ required: true }]}>
            <Input.Password size={'large'} />
          </Form.Item>
          <Form.Item
            name={'confirmPassword'}
            label={'Konfirmasi kata sandi'}
            dependencies={['newPassword']}
            rules={[
              {
                required: true
              },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('newPassword') === value) {
                    return Promise.resolve()
                  }
                  return Promise.reject(
                    new Error('The new password that you entered do not match!')
                  )
                }
              })
            ]}
          >
            <Input.Password size={'large'} />
          </Form.Item>
        </Card>
        <Flex justify={'end'} gap={24}>
          <Link href={'/dashboard/account'} passHref>
            <Button type={'text'} htmlType={'button'} size={'large'}>
              Batalkan
            </Button>
          </Link>
          <Button type={'primary'} htmlType={'submit'} size={'large'}>
            Simpan
          </Button>
        </Flex>
      </Flex>
    </Form>
  )
}

export default EditPassword
