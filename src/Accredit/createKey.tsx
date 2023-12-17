import { Button, Form, Input, Divider, Space } from 'antd'
import { invoke } from '@tauri-apps/api'
import { useCallback, useState } from 'react'
import TextArea from 'antd/es/input/TextArea'

type FieldType = {
  appName?: string
}

export default function CreateKey() {
  const [loading, setLoading] = useState(false)

  const onFinish = useCallback(async (values: FieldType) => {
    setLoading(true)
    await invoke('create_app_keys', {
      appName: values.appName,
    })
    setLoading(false)
  }, [])
  return (
    <>
      <Divider>
        <h3>创建应用密钥</h3>
      </Divider>
      <Form
        onFinish={onFinish}
        style={{
          width: '600px',
        }}
      >
        <Form.Item<FieldType>
          name="appName"
          rules={[{ required: true, message: '输入用户签名!' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" block loading={loading}>
            Submit
          </Button>
        </Form.Item>
        <Form.Item>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
            }}
          >
            <TextArea
              style={{
                height: 200,
              }}
              disabled
            />
            <TextArea
              style={{
                height: 200,
              }}
              disabled
            />
          </div>
        </Form.Item>
      </Form>
    </>
  )
}
