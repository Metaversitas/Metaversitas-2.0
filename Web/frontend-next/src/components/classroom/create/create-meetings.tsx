import React from 'react'
import { Button, Flex, Form, Collapse } from 'antd'
import type { FormInstance } from 'antd'
import Meeting, { HeaderMeeting } from '@/components/classroom/create/meeting'

type TCreateMeeting = {
  form: FormInstance
}

const CreateMeetings = ({ form }: TCreateMeeting) => {
  return (
    <Flex vertical gap={8}>
      <Form
        layout={'vertical'}
        form={form}
        name={'createMeetings'}
        initialValues={{ meetings: [{}] }}
      >
        <Form.List name={'meetings'}>
          {(fields, { add, remove }) => (
            <Flex vertical gap={24}>
              {fields.map((field) => (
                <Collapse
                  key={field.name}
                  expandIconPosition={'end'}
                  collapsible={'icon'}
                  items={[
                    {
                      label: <HeaderMeeting field={field} remove={remove} />,
                      children: <Meeting field={field} />
                    }
                  ]}
                />
              ))}

              <Button type="dashed" onClick={() => add()} block>
                + Tambah Pertemuan
              </Button>
            </Flex>
          )}
        </Form.List>
      </Form>
    </Flex>
  )
}

export default CreateMeetings
