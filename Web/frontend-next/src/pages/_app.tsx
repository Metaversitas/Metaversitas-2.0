import React from 'react'
import { ConfigProvider } from 'antd'
import type { AppProps } from 'next/app'
import '@/styles/globals.css'
import { Plus_Jakarta_Sans } from 'next/font/google'
import type { NextPageWithLayout } from 'next'
import themeConfig from '@/theme/theme-config'

type AppPropsWithLayout<P = AppProps> = AppProps & {
  Component: NextPageWithLayout<P>
}

const plusJakartaSans = Plus_Jakarta_Sans({
  weight: ['500', '600', '700', '800'],
  subsets: ['latin']
})

const App = ({ Component, pageProps }: AppPropsWithLayout) => {
  const getLayout = Component.getLayout ?? ((page) => page)

  return (
    <>
      <style jsx global>{`
        html {
          font-family: ${plusJakartaSans.style.fontFamily};
        }
      `}</style>
      <ConfigProvider theme={themeConfig}>{getLayout(<Component {...pageProps} />)}</ConfigProvider>
    </>
  )
}

export default App
