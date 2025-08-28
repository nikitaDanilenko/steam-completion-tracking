import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import {HashRouter, Route, Routes} from 'react-router-dom'
import Profiles from "./paths/Profiles.tsx";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <HashRouter>
      <Routes>
        <Route path='/'>
          <Route index element={<App/>}/>
          <Route path='profiles/:accountId' element={<Profiles/>}/>
        </Route>
      </Routes>
    </HashRouter>


  </StrictMode>
)
