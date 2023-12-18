import { Button, Form, Input, Divider, message, Space } from 'antd'
import { dialog, invoke } from '@tauri-apps/api'
import { useCallback, useState } from 'react'
import { DownloadOutlined } from '@ant-design/icons'

type FieldType = {
  appName?: string
}

async function chooseSavePath() {
  try {
    const filePath = await dialog.save({
      defaultPath: 'public_key.pem',
    })
    return filePath
  } catch (error) {
    message.error('选择保存路径时出错')
  }
}

export default function CreateKey() {
  const [loading, setLoading] = useState(false)
  const [doLoading, setDoLoading] = useState(false)
  const [dwdisabled, setdwdisabled] = useState(true)

  const [form] = Form.useForm()

  const onFinish = useCallback(async (values: FieldType) => {
    setLoading(true)
    await invoke('create_app_keys', {
      appName: values.appName,
    })
      .then((_) => {
        message.success('创建成功')
      })
      .catch((err: any) => {
        message.error(err)
      })
    setLoading(false)
    setdwdisabled(false)
  }, [])

  const downloadDir = async () => {
    setDoLoading(true)
    chooseSavePath().then((newPath) => {
      invoke('download_pub_key', {
        appName: form.getFieldValue('appName'),
        newPath,
      }).then(() => {
        setDoLoading(false)
        setdwdisabled(true)
        message.success('下次成功')
      })
    })
  }

  return (
    <>
      <Divider>
        <h3>
          <Space>
            <span>Create</span>
            {'<=>'}
            <span>Download</span>
          </Space>
        </h3>
      </Divider>
      <Form
        onFinish={onFinish}
        form={form}
        style={{
          width: '600px',
        }}
      >
        <Form.Item<FieldType>
          name="appName"
          rules={[{ required: true, message: '请输入用户签名!' }]}
        >
          <Input allowClear placeholder="请输入用户签名" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" block loading={loading}>
            Submit
          </Button>
        </Form.Item>
        <Form.Item>
          <Button
            onClick={downloadDir}
            disabled={dwdisabled}
            block
            icon={<DownloadOutlined />}
            loading={doLoading}
          >
            下载公钥 🔑
          </Button>
        </Form.Item>
      </Form>
    </>
  )
}
