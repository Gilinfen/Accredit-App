import React, { useMemo } from 'react'
import { UserOutlined, AccountBookFilled } from '@ant-design/icons'
import type { MenuProps } from 'antd'
import { Layout, Menu, theme } from 'antd'
import { Outlet, useNavigate } from 'react-router-dom'

const { Content, Sider } = Layout

const itemsObj = [
  {
    icon: UserOutlined,
    label: 'RSA 签名',
    link: '/',
  },
  {
    icon: AccountBookFilled,
    label: '应用',
    link: '/applint',
  },
  {
    icon: AccountBookFilled,
    label: 'Base64',
    link: '/base64',
  },
]

const LayoutCom: React.FC = () => {
  const nav = useNavigate()

  const items: MenuProps['items'] = useMemo(
    () =>
      itemsObj.map((item, index) => ({
        key: String(index + 1),
        icon: React.createElement(item.icon),
        label: item.label,
        onClick() {
          nav(item.link)
        },
      })),
    []
  )

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken()

  return (
    <Layout hasSider>
      <Sider
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        <div className="demo-logo-vertical" />
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={['1']}
          items={items}
        />
      </Sider>
      <Layout style={{ marginLeft: 200 }}>
        <Content style={{ margin: '24px 16px 0', overflow: 'initial' }}>
          <div
            style={{
              padding: 24,
              textAlign: 'center',
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Outlet />
          </div>
        </Content>
      </Layout>
    </Layout>
  )
}

export default LayoutCom
