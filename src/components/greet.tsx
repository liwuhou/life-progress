'use client'

import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'

export default function Greet() {
  const [greeting, setGreeting] = useState('')

  useEffect(() => {
    ;(async () => {
      const res = await invoke<string>('greet', { name: 'William' })
      setGreeting(res)
    })()
  }, [])

  return <div>{greeting}</div>
}
