/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    theme: {
        extend: {
            fontFamily: {
                display: ['Unbounded', 'system-ui', 'sans-serif'],
                body: ['Manrope', 'system-ui', 'sans-serif'],
                mono: ['"JetBrains Mono"', 'ui-monospace', 'monospace']
            },
            colors: {
                ink: { DEFAULT: '#0A0E14', 900: '#070A0F', 800: '#0E141C', 700: '#141C28', 600: '#1C2736' },
                paper: { DEFAULT: '#F3F1EA', 800: '#EAE6DB' },
                accent: { DEFAULT: '#F97316', 400: '#FB923C', 600: '#EA580C' },
                steel: { 200: '#DCE4EC', 300: '#C3CFDB', 400: '#93A4B8', 500: '#64748B', 600: '#475569', 700: '#334155' }
            },
            animation: {
                'float': 'float 6s ease-in-out infinite',
                'fade-up': 'fadeUp 0.8s ease-out both',
                'ping': 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
            },
            keyframes: {
                float: {
                    '0%, 100%': { transform: 'translateY(0px)' },
                    '50%': { transform: 'translateY(-12px)' },
                },
                fadeUp: {
                    '0%': { opacity: '0', transform: 'translateY(24px)' },
                    '100%': { opacity: '1', transform: 'translateY(0)' },
                },
            }
        }
    },
    plugins: [],
    darkMode: 'class',
};
