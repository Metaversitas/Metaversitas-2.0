import React from 'react'
import StyledComponentsRegistry from '@/lib/antd-registry'

const Providers = ({ children }: React.PropsWithChildren) => (
  <StyledComponentsRegistry>
    {/*<ConfigProvider theme={themeConfig} >*/}
    {/*<ReactQueryRegistry>*/}
    {children}
    {/*</ReactQueryRegistry>*/}
    {/*</ConfigProvider>*/}
  </StyledComponentsRegistry>
)

export default Providers
