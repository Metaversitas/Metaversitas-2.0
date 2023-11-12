import React from 'react'
import { Col, Flex, Row } from 'antd'
import Link from 'next/link'
// import Class from '@/components/dashboard/class/class'
import Text from '@/components/typography/text'
// import { getClassroom } from '@/lib/api/classroom/get-classroom'

const AvailableClasses = () => {
  return (
    <Flex gap={27} vertical>
      <Row align={'middle'} justify={'space-between'}>
        <Col>
          <Text>Lorem Ipsum *kelas yg udh dibuat</Text>
        </Col>
        <Col>
          <Link href={'/dashboard/classroom'}>Lihat semua</Link>
        </Col>
      </Row>
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
  )
}

export default AvailableClasses
