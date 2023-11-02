import React from 'react'

const Page = ({ params }: { params: { classId: string } }) => {
  return <div>detail class: {params.classId}</div>
}
export default Page
