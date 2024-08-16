'use client'

import { useEffect, useState } from 'react'

const Time = () => {
  const [mounted, setMounted] = useState(false)
  const [time, setTime] = useState(
    new Date().toLocaleString('en-US', {
      timeZone: 'Asia/Yekaterinburg',
      hour: 'numeric',
      minute: 'numeric',
      second: 'numeric',
      hourCycle: 'h23',
    }),
  )

  useEffect(() => {
    setInterval(() => {
      setTime(
        new Date().toLocaleString('en-US', {
          timeZone: 'Asia/Yekaterinburg',
          hour: 'numeric',
          minute: 'numeric',
          second: 'numeric',
          hourCycle: 'h23',
        }),
      )
    }, 1000)
  })

  useEffect(() => setMounted(true), [])

  return <span className='text-overlay0'>{mounted && time}</span>
}

export default Time
