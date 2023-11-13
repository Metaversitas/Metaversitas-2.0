type TRequest = <T>(
  url: string,
  config?: RequestInit
) => Promise<{
  response: Response
  data: T
}>

const originalRequest: TRequest = async (url, config) => {
  const response = await fetch(process.env.NEXT_PUBLIC_BASE_URL + url, config)

  const data = await response.json()
  return { response, data }
}

const fetchInstance = async <T>(url: string, config?: RequestInit) => {
  const { response, data } = await originalRequest<T>(url, config)

  return { response, data }
}

export default fetchInstance
