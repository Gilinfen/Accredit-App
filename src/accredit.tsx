import React, { useCallback, useState } from 'react'
import { Button, Form, Input, Divider } from 'antd'
import { invoke } from '@tauri-apps/api'

const { TextArea } = Input

type FieldType = {
  signature?: string
}

const Accredit: React.FC = () => {
  const [bs_ac, setBsAc] = useState('')
  const [loading, setLoading] = useState(false)
  const onFinish = useCallback(async (values: any) => {
    setLoading(true)
    // 假设 data 是你想要签名的数据，例如一个字符串或二进制数据
    const data = new TextEncoder().encode(values.signature)
    await invoke('create_signature', {
      data: Array.from(data),
    }).then((res: any) => {
      setBsAc(res)
      setLoading(false)
      console.log({
        signature: values.signature,
        base64: res,
      })
    })
  }, [])

  return (
    <div
      style={{
        width: '100%',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        flexWrap: 'wrap',
      }}
    >
      <Divider>
        <h1>创建应用签名</h1>
      </Divider>
      <Form
        onFinish={onFinish}
        style={{
          width: '400px',
        }}
      >
        <Form.Item<FieldType>
          name="signature"
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
          <TextArea
            style={{
              height: 200,
            }}
            value={bs_ac}
          />
        </Form.Item>
      </Form>
    </div>
  )
}

export default Accredit
