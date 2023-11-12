import React, { ReactElement } from 'react'
import LayoutDashboard from '@/components/layout/dashboard'
import EditPassword from '@/components/account/EditPassword'

const Password = () => {
  return <EditPassword />
}

Password.getLayout = function getLayout(page: ReactElement) {
  return <LayoutDashboard>{page}</LayoutDashboard>
}

export default Password
