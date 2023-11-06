'use client'
import React, { CSSProperties } from 'react'
import { useSelectedLayoutSegment } from 'next/navigation'
import { Menu } from 'antd'
import { navHomePage } from '@/lib/layout/navigation-data'
import MenuItem from '@/components/menu/menu-item'

const styleActiveMenu: CSSProperties = {
  borderLeftWidth: 6,
  borderLeftColor: '#5653FC',
  borderLeftStyle: 'solid'
}
const SidebarNavigation = ({ defaultSelectedKeys }: { defaultSelectedKeys: string }) => {
  const segment = useSelectedLayoutSegment()
  const isActive = (path?: React.Key) => segment === path
  console.log('segment', segment)
  return (
    <Menu
      mode="inline"
      defaultSelectedKeys={[defaultSelectedKeys]}
      style={{ backgroundColor: 'transparent', border: 'none' }}
    >
      {navHomePage.map((item) => (
        <MenuItem key={item.key} style={isActive(item.key) ? styleActiveMenu : {}}>
          {item.label}
        </MenuItem>
      ))}
    </Menu>
  )
}

export default SidebarNavigation
