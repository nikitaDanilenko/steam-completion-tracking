import './App.css'
import {useState} from 'react'
import {Link} from 'react-router-dom'

const AccountTypeEnum = {
  Id: 'id',
  Profiles: 'profiles'
} as const

type AccountType = (typeof AccountTypeEnum)[keyof typeof AccountTypeEnum]

const me = '76561197986952100'

function App() {
  const [accountType, setAccountType] = useState<AccountType>(
    AccountTypeEnum.Profiles
  )
  const [accountId, setAccountId] = useState<string>(me)
  const [steamToken, setSteamToken] = useState<string>('')

  const storeToken = (token: string) => {
    localStorage.setItem('sct-token', token)
  }

  const setAndStoreToken = (token: string) => {
    setSteamToken(token)
    storeToken(token)
  }

  return (
    <main>
      <header>
        <h1>Steam Completion Tracker</h1>
        <p>
          Steam Cookie:
          <input type="text" id="steam-token" name="steam-token"
                 value={steamToken}
                 onChange={e => setAndStoreToken(e.target.value)}/>
        </p>
        <p>
          <label htmlFor="account-type">Account Path:</label>
          <select
            id="account-type"
            value={accountType}
            onChange={e => setAccountType(e.target.value as AccountType)}
          >
            <option value={AccountTypeEnum.Id}>id</option>
            <option value={AccountTypeEnum.Profiles}>profiles</option>
          </select>
          /
          <input type="text" id="account-id" name="account-id"
                 value={accountId}
                 onChange={e => setAccountId(e.target.value)}/>

          <Link to={`/${accountType}/${accountId}`}>Fetch</Link>

        </p>
      </header>
    </main>
  )
}

export default App
