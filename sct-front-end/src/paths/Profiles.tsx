import {useParams} from 'react-router-dom';

function Profiles() {
  const {accountId} = useParams<{ accountId: string }>()
  const page =
    <main>
      <h1>Profiles</h1>
      <p>This is the Profiles page for {accountId}.</p>
    </main>

  return page
}

export default Profiles
