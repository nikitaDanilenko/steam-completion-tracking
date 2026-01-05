import {useParams} from 'react-router-dom';
import {useEffect, useState} from "react";
import appConfig from "../config.ts";

function Profiles() {
  const {accountId} = useParams<{ accountId: string }>()
  const steamToken = localStorage.getItem('sct-token') || ''
  const [profileName, setProfileName] = useState('loading...')
  const [games, setGames] = useState([])

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch(
        `${appConfig.api.url}steam-list/profiles/${accountId}`,
        {
          headers: {
            'steam-token': steamToken
          }
        }
      )
      const data = await response.json()
      setProfileName(data.profileName)
      setGames(data.games || [])
    }
    fetchData().then(r => console.log(r))
  }, [accountId, steamToken])

  const page =
    <main>
      <h1>Profiles</h1>
      <p>This is the Profiles page for {accountId}, current token is {steamToken}.</p>
      <h2>Profile name: {profileName}</h2>
      <h3>Games owned: {games.length}</h3>
    </main>

  return page
}

export default Profiles
