import { Button, Divider, Space, message } from 'antd'
import TextArea from 'antd/es/input/TextArea'
import { useCallback, useState } from 'react'
import { copyTextToClipboard } from '../../utils'
import { invoke } from '@tauri-apps/api'

const CodeCentext = ({ type }: { type: 'encode_str' | 'decode_str' }) => {
  const [value, setValue] = useState<string>()

  const click = useCallback(async () => {
    if (!value) return

    invoke(type, {
      string: value,
    })
      .then((res: any) => {
        copyTextToClipboard(res)
        message.success('复制成功')
      })
      .catch((err: any) => {
        message.error(err)
      })
  }, [value, type])

  return (
    <>
      <Divider>
        <h1>{type === 'encode_str' ? 'Encode' : 'Decode'}</h1>
      </Divider>
      <Space
        style={{
          width: 400,
        }}
        direction="vertical"
      >
        <TextArea
          value={value}
          style={{
            height: 200,
          }}
          onChange={(e) => setValue(e.target.value)}
        />
        <Button type="primary" block onClick={click}>
          Copy
        </Button>
      </Space>
    </>
  )
}

export default function Base64App() {
  return (
    <div className="context-center">
      <CodeCentext type="encode_str" />
      <CodeCentext type="decode_str" />
    </div>
  )
}
