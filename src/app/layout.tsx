import './global.css'
import { cn } from '@/lib/utils'
import { Metadata, Viewport } from 'next'
import localFont from 'next/font/local'

const cartographcf = localFont({
  variable: '--font-cartographcf',
  src: [
    {
      path: '../../public/fonts/CartographCF-Regular.woff2',
      weight: '400',
      style: 'normal',
    },
    {
      path: '../../public/fonts/CartographCF-ExtraBold.woff2',
      weight: '800',
      style: 'normal',
    },
  ],
})

export const viewport: Viewport = {
  themeColor: '#161616',
  initialScale: 1,
  maximumScale: 1,
}

export const metadata: Metadata = {
  metadataBase: new URL('https://mariesavch.vercel.app'),
  title: {
    default: 'marie',
    template: '%s | marie',
  },
  description: 'frontend developer',
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  twitter: {
    title: {
      template: '%s | marie',
      default: 'marie',
    },
    card: 'summary_large_image',
    description: 'frontend developer',
    images: '/api/og?heading=marie&desc=frontend%20developer',
  },
  openGraph: {
    title: {
      template: '%s | marie',
      default: 'marie',
    },
    description: 'frontend developer',
    siteName: 'marie',
    locale: 'en-US',
    type: 'website',
    url: 'https://mariesavch.vercel.app/',
    images: '/api/og?heading=marie&desc=frontend%20developer',
  },
  icons: {
    icon: '/favicon.png',
    apple: 'apple-touch-icon.png',
  },
  alternates: {
    canonical: 'https://mariesavch.vercel.app/',
  },
  manifest: '/site.webmanifest',
  other: {
    'darkreader-lock': '',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html
      lang='en'
      className={cn(
        'scroll-smooth font-sans text-text bg-base',
        'antialiased selection:bg-surface1 leading-relaxed',
        cartographcf.variable,
      )}
    >
      <body>
        <div className='mx-auto max-w-3xl px-6 pb-20'>
          <main className='pt-16'>
            {children}
          </main>
        </div>
      </body>
    </html>
  )
}
