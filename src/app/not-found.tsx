import Link from 'next/link'

export default function NotFound() {
  return (
    <div>
      <h1 className='text-xl font-bold mb-2'>404 - page not found</h1>
      <p className='text-overlay0 mb-5'>
        nothing here, maybe you misspelled the link
      </p>
      <Link href='/' className='underlined'>go to home</Link>
    </div>
  )
}
