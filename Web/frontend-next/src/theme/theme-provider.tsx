'use client'
import React, { PropsWithChildren } from 'react'
import { ConfigProvider } from 'antd'
import themeConfig from '@/theme/theme-config'

const ThemeProvider = ({ children }: PropsWithChildren) => {
  return <ConfigProvider theme={themeConfig}>{children}</ConfigProvider>
}

export default ThemeProvider
