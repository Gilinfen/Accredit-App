import { CopyOutlined } from '@ant-design/icons'
import { message } from 'antd'

export default function Copy({ onClick, ...pre }: { onClick?: () => void }) {
  return (
    <>
      <CopyOutlined
        className="copy-icno"
        {...pre}
        onClick={() => {
          onClick?.()
          message.success('复制成功')
        }}
      />
    </>
  )
}
