import React, { ReactElement, useState } from 'react'
import { Button, Flex, Form, message, Steps, theme } from 'antd'
import { useRouter } from 'next/router'
import dynamic from 'next/dynamic'
import LayoutDashboard from '@/components/layout/dashboard'

const CreateClass = dynamic(() => import('@/components/classroom/create/create-class'), {
  ssr: false
})
const CreateMeetings = dynamic(() => import('@/components/classroom/create/create-meetings'), {
  ssr: false
})

// import CreateClass from '@/components/classroom/create/create-class'
// import CreateMeetings from '@/components/classroom/create/create-meetings'
// const initialValues = {
//   class_name: undefined,
//   secondary_subject_id: undefined,
//   semester: undefined,
//   subject_id: undefined,
//   teachers: undefined,
//   year_end: undefined,
//   year_start: undefined,
//   meetings: [{}]
// }

const Create = () => {
  const [form] = Form.useForm()
  const { token } = theme.useToken()
  const [current, setCurrent] = useState(1)
  const router = useRouter()

  const [formClass] = Form.useForm()
  const [formMeeting] = Form.useForm()

  const steps = [
    {
      title: 'Kelas',
      content: <CreateClass form={formClass} />
    },
    {
      title: 'Pertemuan',
      content: <CreateMeetings form={formMeeting} />
    }
  ]
  const next = () => {
    formClass.validateFields().then((value) => {
      console.log(value)
      setCurrent(current + 1)
    })
  }

  const prev = () => {
    setCurrent(current - 1)
  }

  const items = steps.map((item) => ({ key: item.title, title: item.title, icon: <></> }))

  const contentStyle: React.CSSProperties = {
    color: token.colorTextTertiary,
    backgroundColor: 'transparent',
    borderRadius: token.borderRadiusLG,
    border: `none`,
    marginTop: 16
  }

  const onCancel = (current: number) => {
    if (current > 0) {
      prev()
    } else {
      router.push('/dashboard/classroom')
    }
  }

  // Watch all values
  const onFinish = () => {
    const valuesClass = formClass.getFieldsValue(true)
    const valuesMeetings = formMeeting.getFieldsValue(true)
    console.log('onFinish value', { ...valuesClass, ...valuesMeetings })
    message.success('Processing complete!')
  }

  // const [submittable, setSubmittable] = React.useState(false)

  // React.useEffect(() => {
  //   // form.validateFields({ validateOnly: true }).then(
  //   //   (value) => {
  //   //     console.log(values, value)
  //   //     setSubmittable(true)
  //   //   },
  //   //   () => {
  //   //     setSubmittable(false)
  //   //   }
  //   // )
  //   schema
  //     .isValid(values)
  //     .then(function (valid) {
  //       console.log(valid, values)
  //       setSubmittable(false)
  //     })
  //     .catch((err) => {
  //       console.log(err)
  //       setSubmittable(false)
  //     })
  // }, [values])

  return (
    <Form onFinish={onFinish} layout={'vertical'} form={form} name={'create'}>
      <Flex vertical gap={24}>
        <Steps current={current} items={items} type="navigation" />
        <div style={contentStyle}>{steps[current].content}</div>
        <Flex gap={24} justify={'end'}>
          <Button
            htmlType={'button'}
            type={'text'}
            onClick={() => onCancel(current)}
            size={'large'}
          >
            Batalkan
          </Button>
          {current < steps.length - 1 && (
            <Button htmlType={'button'} type="primary" onClick={() => next()} size={'large'}>
              Buat Kelas
            </Button>
          )}
          {current === steps.length - 1 && (
            <Button htmlType={'submit'} type="primary" size={'large'}>
              Buat Kelas
            </Button>
          )}
        </Flex>
      </Flex>
    </Form>
  )
}

Create.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}

export default Create
