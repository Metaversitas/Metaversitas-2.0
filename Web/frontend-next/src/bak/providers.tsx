import React from 'react'
import { ConfigProvider } from 'antd'
import StyledComponentsRegistry from '@/lib/antd-registry'
import themeConfig from '@/theme/theme-config'

const Providers = ({ children }: React.PropsWithChildren) => (
  <StyledComponentsRegistry>
    <ConfigProvider theme={themeConfig}>
      {/*<ReactQueryRegistry>*/}
      {children}
      {/*</ReactQueryRegistry>*/}
    </ConfigProvider>
  </StyledComponentsRegistry>
)

export default Providers
