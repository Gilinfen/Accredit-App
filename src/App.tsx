import React, { useEffect } from 'react'
import Accredit from './Accredit'
import { invoke } from '@tauri-apps/api'

const App: React.FC = () => {
  useEffect(() => {
    // 页面加载完成后通知 Tauri 显示窗口
    invoke('app_ready')
  }, [])
  return <Accredit />
}

export default App
