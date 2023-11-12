import React from 'react'
import Link from 'next/link'
import { Button, Checkbox, Col, Flex, Row, Select } from 'antd'
import ThemeProvider from '@/theme/theme-provider'
import InputSearch from '@/components/input/input-search'
// import Class from '@/components/dashboard/class/class'
import Option from '@/components/select/option'

const Page = () => {
  return (
    <ThemeProvider>
      <Flex gap={52} vertical>
        <Row justify={'center'}>
          <Col xs={24} sm={22} md={18} lg={20}>
            <Row gutter={24}>
              <Col flex={1}>
                <InputSearch />
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
          {/*<Row>*/}
          {/*  {[*/}
          {/*    {*/}
          {/*      capacity: 50,*/}
          {/*      class_id: 'b5ed6748-911d-4cc0-8142-cf82c42ca1e0',*/}
          {/*      current_meeting_id: null,*/}
          {/*      description: null,*/}
          {/*      end_time: null,*/}
          {/*      have_multiple_meeting: false,*/}
          {/*      is_active: true,*/}
          {/*      meetings: [],*/}
          {/*      name: 'Physics',*/}
          {/*      semester: 'odd',*/}
          {/*      start_time: null,*/}
          {/*      subject_id: 'd24a4a07-9576-48dd-ad6b-daad5fe05058',*/}
          {/*      subject_name: 'Physics',*/}
          {/*      teachers: [*/}
          {/*        {*/}
          {/*          class_id: 'b5ed6748-911d-4cc0-8142-cf82c42ca1e0',*/}
          {/*          teacher_id: 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae',*/}
          {/*          teacher_name: 'Nama Lengkap Dosen'*/}
          {/*        },*/}
          {/*        {*/}
          {/*          class_id: 'b5ed6748-911d-4cc0-8142-cf82c42ca1e',*/}
          {/*          teacher_id: 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74a',*/}
          {/*          teacher_name: 'Nama Lengkap Dosen'*/}
          {/*        }*/}
          {/*      ],*/}
          {/*      year_end: '2024',*/}
          {/*      year_start: '2023'*/}
          {/*    }*/}
          {/*  ].map((item) => (*/}
          {/*    <Col key={item.class_id} span={24}>*/}
          {/*      <Class*/}
          {/*        subject={item.subject_name}*/}
          {/*        topic={item.name}*/}
          {/*        lecturer={item.teachers.map((teacher) => teacher.teacher_name).join(', ')}*/}
          {/*        classId={item.class_id}*/}
          {/*        isEnroll={false}*/}
          {/*      />*/}
          {/*    </Col>*/}
          {/*  ))}*/}
          {/*</Row>*/}
        </Flex>
      </Flex>
    </ThemeProvider>
  )
}

export default Page
