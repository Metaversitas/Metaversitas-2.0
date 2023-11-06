import type { ThemeConfig } from 'antd'
import { AliasToken } from 'antd/es/theme/interface'

const font: Partial<AliasToken> = {
  fontSize: 16,
  fontSizeLG: 24,
  fontSizeXL: 32,
  fontSizeHeading1: 64,
  fontSizeHeading2: 56,
  fontSizeHeading3: 48,
  fontSizeHeading4: 40,
  fontFamily: 'inherit'
}

const color: Partial<AliasToken> = {
  colorPrimary: 'rgb(252, 83, 83)',
  colorSuccess: '#2ec52b',
  colorWarning: '#ffb800',
  colorError: '#ed2222',
  colorInfo: '#3c1b46'
  // colorBgContainer: 'inherit'
}

const components = {
  Typography: {
    fontSizeHeading1: font.fontSizeHeading1,
    fontSizeHeading2: font.fontSizeHeading2,
    fontSizeHeading3: font.fontSizeHeading3,
    fontSizeHeading4: font.fontSizeHeading4,
    colorTextSecondary: '#9C9C9C',
    titleMarginBottom: '0',
    titleMarginTop: '0'
  },
  Layout: {
    bodyBg: '#fff',
    footerBg: '#FFE3E3',
    headerBg: '#FFE3E3',
    siderBg: '#F5F5FA'
  },
  Menu: {
    itemSelectedColor: 'rgb(26, 26, 26)',
    itemSelectedBg: 'rgb(241, 241, 241)',
    itemHoverColor: 'rgb(26, 26, 26)',
    itemHoverBg: 'rgb(241, 241, 241)',
    itemColor: 'rgb(26, 26, 26)'
  },
  Button: {
    defaultBorderColor: 'rgb(252, 83, 83)',
    colorPrimary: 'rgb(252, 83, 83)',
    defaultBg: 'transparent'
  }
}

const themeConfig: ThemeConfig = {
  token: {
    ...font,
    ...color
  },
  components
}

export default themeConfig
