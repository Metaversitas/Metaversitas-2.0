'use client'

import React, { ComponentType, ReactNode } from 'react'
import { ConfigProvider } from 'antd'
import themeConfig from '@/theme/theme-config'

type TProps = {
  children?: ReactNode
}
const getDisplayName = <P extends TProps>(WrappedComponent: ComponentType<P>) => {
  return WrappedComponent.displayName || WrappedComponent.name || 'Component'
}

const withTheme = <P extends Record<string, unknown>>(
  Component: ComponentType<P>
): ComponentType<P> => {
  const WrapperTheme: ComponentType<P> = (props) => {
    Component.displayName = `Component(${getDisplayName(Component)})`

    return (
      <>
        <ConfigProvider theme={themeConfig}>
          <Component {...props} />
        </ConfigProvider>
      </>
    )
  }

  return WrapperTheme
}

withTheme.displayName = `withTheme`

export default withTheme
