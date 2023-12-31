import { Route, Routes } from 'react-router-dom'
import App from '../App'
import AppList from '../page/appList'
import Base64App from '../page/base64'
import Layout from '../layout'

function RouterComponent() {
  return (
    <Routes>
      <Route path={'/'} element={<Layout />}>
        <Route index element={<App />} />
        <Route path="/applint" element={<AppList />} />
        <Route path="/base64" element={<Base64App />} />
      </Route>
    </Routes>
  )
}

export default RouterComponent
