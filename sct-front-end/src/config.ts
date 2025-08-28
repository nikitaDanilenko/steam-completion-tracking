interface ApiConfig {
  url: string;
}

export interface AppConfig {
  api: ApiConfig;
}

const appConfig: AppConfig = {
  api: {
    url: import.meta.env.VITE_API_URL,
  },
}

const requiredEnvironmentVariables = [
  'VITE_API_URL',
]

const missingVariables = requiredEnvironmentVariables.filter(
  variable => !import.meta.env[variable]
)

if (missingVariables.length > 0) {
  throw new Error(
    `Missing required environment variables: ${missingVariables.join(', ')}`
  );
}

export default appConfig;
