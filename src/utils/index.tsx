import { dialog, invoke } from '@tauri-apps/api'
import { message } from 'antd'

export async function copyTextToClipboard(text: string): Promise<void> {
  try {
    await navigator.clipboard.writeText(text)
  } catch (err) {}
}

export type SecretKey = 'private_key.pem' | 'public_key.pem'

export async function chooseSavePath(
  type: SecretKey,
  data: {
    appName: string
  }
) {
  try {
    const filePath = await dialog.save({
      defaultPath: type,
    })
    invoke('download_secret_key', { ...data, newPath: filePath, key: type })
  } catch (error) {
    message.error('选择保存路径时出错')
  }
}
