import React, { ReactElement } from 'react'
import LayoutDashboard from '@/components/layout/dashboard'

const DetailClassRoom = () => {
  return <div></div>
}
DetailClassRoom.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}

export default DetailClassRoom
