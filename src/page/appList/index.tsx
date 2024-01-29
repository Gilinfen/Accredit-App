import { invoke } from '@tauri-apps/api'
import { Button, Card, Divider, Space, Empty } from 'antd'
import { useEffect, useState } from 'react'
import { SecretKey, chooseSavePath, copyTextToClipboard } from '../../utils'
import Copy from '../../components/copy'
import { DownloadOutlined } from '@ant-design/icons'

interface Signature {
  base_code: string
  use_info: string
}
interface AppInfo {
  app_name: string
  app_name_path: string
  pri_key_path: string
  pub_key_puth: string
  signature: Signature[]
}

const DownloadKey = ({ appName }: { appName: string }) => {
  const downloadDir = async (type: SecretKey) => {
    chooseSavePath(type, {
      appName,
    })
  }
  return (
    <Space>
      <Button
        onClick={() => downloadDir('private_key.pem')}
        icon={<DownloadOutlined />}
      >
        私钥 🔑
      </Button>
      <Button
        type="primary"
        onClick={() => downloadDir('public_key.pem')}
        icon={<DownloadOutlined />}
      >
        公钥 🔑
      </Button>
    </Space>
  )
}

export default function AppList() {
  const [applist, setAPplist] = useState<AppInfo[]>([])
  useEffect(() => {
    invoke('get_app_info_json').then((res: any) => {
      setAPplist(res)
    })
  }, [])

  return (
    <>
      {applist.map((item) => {
        return (
          <Card
            key={item.app_name}
            title={item.app_name}
            extra={<DownloadKey appName={item.app_name} />}
            type="inner"
            className="app-list-card"
          >
            {item.signature.map((val) => (
              <div>
                <Divider>
                  <h3>{val.use_info}</h3>
                </Divider>
                <Button
                  type="link"
                  onClick={() => copyTextToClipboard(val.use_info)}
                  icon={<Copy />}
                >
                  Copy 标签
                </Button>
                <Button
                  type="link"
                  onClick={() => copyTextToClipboard(val.base_code)}
                  icon={<Copy />}
                >
                  Copy 激活码
                </Button>
              </div>
            ))}
            {!item.signature.length && <Empty />}
          </Card>
        )
      })}
      {!applist.length && <Empty />}
    </>
  )
}
