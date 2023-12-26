import React, {
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
} from 'react'
import { Button, Form, Input, Divider, Select } from 'antd'
import { invoke } from '@tauri-apps/api'
import CreateKey from './createKey'
import Verify from './verify'
import { getU8Arr } from '../utils'

const { TextArea } = Input

export type SelectAppNameRef = {
  appName?: string
}

export const SelectAppName = React.forwardRef<
  SelectAppNameRef,
  {
    onChange?: (value: string) => void
  }
>((props, ref) => {
  const [appNames, setAppNames] = useState<
    {
      value: string
      label: string
      disabled?: undefined
    }[]
  >()

  const [appNameVal, setAppNameVal] = useState<string>()

  const handleChange = (value: string) => {
    props.onChange?.(value)
    console.log(`selected ${value}`)
    setAppNameVal(value)
  }

  useEffect(() => {
    getAppNames()
  }, [])

  const getAppNames = useCallback(() => {
    invoke('get_app_names').then((res) => {
      setAppNames(
        (res as string[]).map((item) => ({
          value: item,
          label: item,
        }))
      )
    })
  }, [])

  useImperativeHandle(
    ref,
    () => ({
      appName: appNameVal,
    }),
    [appNameVal]
  )

  return (
    <Select
      placeholder="选择应用"
      value={appNameVal}
      style={{ width: '600px', marginBottom: 24 }}
      onChange={handleChange}
      options={appNames}
      allowClear
      onFocus={getAppNames}
      onClear={() => setAppNameVal(void 0)}
    />
  )
})

type FieldType = {
  signature?: string
}

const Accredit: React.FC = () => {
  const [signature, setSignature] = useState('')
  const [loading, setLoading] = useState(false)
  const [formDisabled, setFormDisabled] = useState(true)

  const appNameRef = useRef<SelectAppNameRef>(null)

  const onFinish = useCallback(async (values: FieldType) => {
    setLoading(true)

    await invoke('create_signature', {
      data: getU8Arr(values.signature),
      appName: appNameRef.current?.appName,
    }).then((res: any) => {
      setSignature(res)
      setLoading(false)
      // console.log({
      //   signature: values.signature,
      //   base64: res,
      // })
    })
  }, [])

  return (
    <div className="context-center">
      <CreateKey />
      <Divider>
        <h3>创建应用签名</h3>
      </Divider>
      <div>
        <SelectAppName
          ref={appNameRef}
          onChange={(value) => {
            setFormDisabled(!value)
          }}
        />
        <Form
          name="create"
          onFinish={onFinish}
          disabled={formDisabled}
          style={{
            width: '600px',
          }}
        >
          <Form.Item<FieldType>
            name="signature"
            rules={[{ required: true, message: '请输入用户标识!' }]}
          >
            <Input placeholder="请输入用户标识" allowClear />
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
              // disabled
              value={signature}
            />
          </Form.Item>
        </Form>
      </div>
      <Verify />
    </div>
  )
}

export default Accredit
