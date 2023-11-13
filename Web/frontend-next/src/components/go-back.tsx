import { useRouter } from 'next/router'
import { Button } from 'antd'

const GoBack = () => {
  const router = useRouter()
  const onBack = () => {
    return router.back()
  }
  return (
    <Button type={'text'} role={'button'} onClick={onBack}>
      {'<'}
    </Button>
  )
}

export default GoBack
