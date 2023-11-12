import Icon from '@ant-design/icons'
import type { CSSProperties, SVGProps } from 'react'
import React from 'react'
import type { CollapseProps } from 'antd'
import { Collapse as AntCollapse, theme } from 'antd'

const ArrowRight = ({ style }: SVGProps<SVGElement>) => {
  return (
    <svg
      width="42"
      height="42"
      viewBox="0 0 42 42"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      style={style}
    >
      <path
        d="M17.5 29.75L26.25 21L17.5 12.25"
        stroke="#1A1A1A"
        strokeWidth="4"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}
const getItems: (
  panelStyle: CSSProperties,
  items: CollapseProps['items']
) => CollapseProps['items'] = (panelStyle, items = []) =>
  items.map((item) => ({
    ...item,
    style: panelStyle,
    className: item.children ? '' : 'no-children'
  }))

const Collapse: React.FC<CollapseProps> = ({ items, ...props }) => {
  const { token } = theme.useToken()

  const panelStyle: React.CSSProperties = {
    marginBottom: 24,
    background: '#fff',
    borderRadius: token.borderRadiusLG,
    border: 'none',
    padding: '29px 25px',
    fontSize: token.fontSizeXL,
    fontWeight: token.fontWeightStrong,
    alignItems: 'center'
  }

  return (
    <>
      <style jsx>{`
        .no-children .ant-collapse-content-box {
          padding: 0;
        }
      `}</style>
      <AntCollapse
        bordered={false}
        defaultActiveKey={['1']}
        expandIcon={({ isActive }) => <Icon component={ArrowRight} rotate={isActive ? 90 : 0} />}
        style={{ background: 'inherit' }}
        items={getItems(panelStyle, items)}
        {...props}
      />
    </>
  )
}

export default Collapse
