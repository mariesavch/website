import Time from '@/components/time'

const Home = () => (
  <section>
    <div>
      <h1 className='mb-2 text-xl font-bold'>marie 🏳️‍⚧️</h1>
      <div className='text-overlay0 mb-5 flex space-x-2'>
        <span>she/her, frontend developer</span> <span>-</span> <Time />
      </div>
    </div>
    <ul className='animated-list grid grid-cols-1 sm:grid-cols-2'>
      <li>
        <div className='flex py-3 flex-col gap-1'>
          <span className='text-overlay0'>
            github
          </span>
          <a className='underlined' href='https://github.com/mariesavch'>mariesavch</a>
        </div>
      </li>
      <li>
        <div className='flex py-3 flex-col gap-1'>
          <span className='text-overlay0'>
            mail
          </span>
          <a className='underlined' href='mailto:mariesavch@icloud.com'>mariesavch@icloud.com</a>
        </div>
      </li>
    </ul>
  </section>
)

export default Home
