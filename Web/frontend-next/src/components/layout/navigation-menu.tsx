'use client'
import React, { useEffect, useState } from 'react'
import { usePathname } from 'next/navigation'
import { Menu } from 'antd'
import { TMenuItem } from '@/lib/layout/navigation-data'

type TNavigationMenu = { items: TMenuItem[]; defaultSelectedKeys: string }
const NavigationMenu = ({ items, defaultSelectedKeys }: TNavigationMenu) => {
  const pathname = usePathname()
  const [selectedKeys, setSelectedKeys] = useState<string>(pathname || defaultSelectedKeys)

  useEffect(() => {
    setSelectedKeys(pathname)
  }, [pathname])
  return (
    <Menu
      mode="horizontal"
      defaultSelectedKeys={[selectedKeys]}
      items={items}
      style={{ backgroundColor: 'transparent', border: 'none' }}
      selectedKeys={[selectedKeys]}
    />
  )
}

export default NavigationMenu
