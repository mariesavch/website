import { ImageResponse } from 'next/og'

export const runtime = 'edge'

const cartographcf = fetch(
  new URL('../../../../public/fonts/CartographCF-Heavy.otf', import.meta.url),
).then(res => res.arrayBuffer())

export async function GET(req: Request) {
  try {
    const fontBold = await cartographcf

    const url = new URL(req.url)
    const values = Object.fromEntries(url.searchParams)
    const heading = values.heading.length > 140
      ? `${values.heading.substring(0, 140)}...`
      : values.heading

    return new ImageResponse(
      (
        <div
          tw='flex relative flex-col px-30 py-30 w-full h-full text-white'
          style={{
            background: '#161616',
          }}
        >
          <div
            tw={`flex leading-[0.9] text-${heading.length >= 15 ? '7xl' : '9xl'} mb-12`}
            style={{
              fontFamily: 'Cartograph CF',
              fontWeight: '800',
            }}
          >
            {heading}
          </div>
          <div
            tw='text-4xl text-[#7a7a7a]'
            style={{ fontFamily: 'Cartograph CF', fontWeight: '800' }}
          >
            {values.desc}
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
        fonts: [
          {
            name: 'Cartograph CF',
            data: fontBold,
            weight: 900,
            style: 'normal',
          },
        ],
      },
    )
  } catch (error) {
    return new Response(`Failed to generate image`, {
      status: 500,
    })
  }
}
