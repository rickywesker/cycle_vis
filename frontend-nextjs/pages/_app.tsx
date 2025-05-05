import '../styles/output.css';
import type { AppProps } from 'next/app';

// This file should only export the App component
export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}