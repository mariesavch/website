import type { Config } from 'tailwindcss'
import { fontFamily } from 'tailwindcss/defaultTheme'

export default {
  content: ['./src/**/*.{ts,tsx,mdx,mdx}'],
  theme: {
    extend: {
      colors: {
        red: 'rgb(var(--red))',
        pink: 'rgb(var(--pink))',
        mauve: 'rgb(var(--pink))',
        peach: 'rgb(var(--peach))',
        yellow: 'rgb(var(--yellow))',
        green: 'rgb(var(--green))',
        teal: 'rgb(var(--teal))',
        blue: 'rgb(var(--blue))',
        text: 'rgb(var(--text))',
        subtext1: 'rgb(var(--subtext1))',
        subtext0: 'rgb(var(--subtext0))',
        overlay2: 'rgb(var(--overlay2))',
        overlay1: 'rgb(var(--overlay1))',
        overlay0: 'rgb(var(--overlay0))',
        surface2: 'rgb(var(--surface2))',
        surface1: 'rgb(var(--surface1))',
        surface0: 'rgb(var(--surface0))',
        base: 'rgb(var(--base))',
        mantle: 'rgb(var(--mantle))',
        crust: 'rgb(var(--crust))',
      },
      fontFamily: {
        sans: ['var(--font-cartographcf)', ...fontFamily.sans],
        mono: ['var(--font-cartographcf)', ...fontFamily.mono],
      },
      fontWeight: {
        semibold: '700',
        bold: '800',
        extrabold: '900',
      },
    },
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
} satisfies Config
