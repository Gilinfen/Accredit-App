import { Button, Form, Input, Divider, message } from 'antd'
import { invoke } from '@tauri-apps/api'
import { useCallback, useState } from 'react'

type FieldType = {
  appName?: string
}

export default function CreateKey() {
  const [loading, setLoading] = useState(false)
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
  }, [])

  return (
    <>
      <Divider>
        <h3>创建应用密钥</h3>
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
          rules={[{ required: true, message: '请输入应用名称!' }]}
        >
          <Input allowClear placeholder="请输入用户签名" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" block loading={loading}>
            Create
          </Button>
        </Form.Item>
      </Form>
    </>
  )
}
