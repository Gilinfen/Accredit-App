import { Button, Form, Input, Divider, Space } from 'antd'
import { invoke } from '@tauri-apps/api'
import { useCallback, useRef, useState } from 'react'
import TextArea from 'antd/es/input/TextArea'
import { SelectAppName, SelectAppNameRef, getU8Arr } from '.'

type FieldType = {
  signature?: string
  user_signature?: string
}

export default function Verify() {
  const [formDisabled, setFormDisabled] = useState(true)
  const appNameRef = useRef<SelectAppNameRef>(null)

  const onFinish = useCallback(
    async (values: FieldType) => {
      console.log(appNameRef.current?.appName, 'appNameRef.current?.appName')

      await invoke('get_verify_signature', {
        appName: appNameRef.current?.appName,
        data: getU8Arr(values.user_signature),
        signature: getU8Arr(values.signature),
      })
    },
    [appNameRef.current]
  )

  return (
    <>
      <Divider>
        <h3>验证应用签名</h3>
      </Divider>
      <SelectAppName
        ref={appNameRef}
        onChange={(value) => {
          setFormDisabled(!value)
        }}
      />
      <Form
        name="verify"
        onFinish={onFinish}
        disabled={formDisabled}
        style={{
          width: '600px',
        }}
      >
        <Form.Item<FieldType>
          name="user_signature"
          rules={[{ required: true, message: '请输入用户标识!' }]}
        >
          <Input placeholder="请输入用户标识" allowClear />
        </Form.Item>
        <Form.Item<FieldType>
          name="signature"
          rules={[{ required: true, message: '请输入应用签名!' }]}
        >
          <TextArea
            style={{
              height: 200,
            }}
            allowClear
            placeholder="请输入应用签名"
          />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" block>
            Submit
          </Button>
        </Form.Item>
      </Form>
    </>
  )
}
