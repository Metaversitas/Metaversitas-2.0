import React, { ReactElement, useEffect, useState } from 'react'
import Link from 'next/link'

import { Button, Checkbox, Col, Flex, Input, Row, Select } from 'antd'
import { AxiosResponse } from 'axios'
import Class from '@/components/dashboard/class/class'
import LayoutDashboard from '@/components/layout/dashboard'
import { Datum, getClassroom } from '@/lib/api/classroom/get-classroom'

const { Option } = Select
const { Search } = Input

const Index = () => {
  const [classroom, setClassroom] = useState<Datum[]>([])
  useEffect(() => {
    ;(async () => {
      const response = (await getClassroom()) as AxiosResponse
      setClassroom(response.data.data)
    })()
  }, [])
  console.log(classroom)
  return (
    <>
      <Flex gap={52} vertical>
        <Row justify={'center'}>
          <Col xs={24} sm={22} md={18} lg={20}>
            <Row gutter={24}>
              <Col flex={1}>
                <Search />
              </Col>
              <Col>
                <Link href={'/dashboard/classroom'}>
                  <Button type={'primary'}>Cari</Button>
                </Link>
              </Col>
            </Row>
          </Col>
        </Row>
        <Flex vertical gap={27}>
          <Flex vertical gap={16}>
            <Row justify={'space-between'} align={'middle'}>
              <Col>
                <Select placeholder={'Jenis Kelas'} style={{ width: 190 }}>
                  <Option value={'fisika'}>Fisika</Option>
                  <Option value={'museum'}>Museum</Option>
                  <Option value={'candi'}>Candi</Option>
                </Select>
              </Col>
              <Col>
                <Checkbox>Owned</Checkbox>
              </Col>
            </Row>
            <Row justify={'space-between'}>
              <Col></Col>
              <Col>
                <Link href={'/dashboard/classroom/create'}>
                  <Button type={'primary'}>Buat Kelas</Button>
                </Link>
              </Col>
            </Row>
          </Flex>
          <Row>
            {classroom.length > 0 &&
              classroom.map((item) => (
                <Col key={item.class_id} span={24}>
                  <Class
                    subject={item.subject_name}
                    topic={item.class_name}
                    lecturer={item.teachers.map((teacher) => teacher.teacher_name).join(', ')}
                    classId={item.class_id}
                    pta={`${item.year_start}/${item.year_end}`}
                    isEnroll={item.is_enrolled}
                  />
                </Col>
              ))}
          </Row>
        </Flex>
      </Flex>
    </>
  )
}

// export const getServerSideProps: GetServerSideProps<TPage> = async () => {
//   const res = await getClassroom()
//   const classroom = res.data
//   return { props: { classroom } }
// }

Index.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}

export default Index
