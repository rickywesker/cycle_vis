import { useState, useEffect } from 'react';
import { apiBase } from '../utils/api';
import axios from 'axios';
import dynamic from 'next/dynamic';
import RSIChart from '../components/RSIChart';
import Navbar from '../components/Navbar';
// Avoid SSR issues with Chart.js
const NoSSRChart = dynamic(() => Promise.resolve(RSIChart), { ssr: false });

interface IndicatorResult {
  symbol: string;
  value: number;
  category: string;
}

export default function Home() {
  const [data, setData] = useState<IndicatorResult[]>([]);
  const [filter, setFilter] = useState('');

  useEffect(() => {
    async function fetchData() {
      try {
        const res = await axios.get<IndicatorResult[]>(`${apiBase}/api/rsi`);
        setData(res.data);
      } catch (err) {
        console.error('Fetch error:', err);
      }
    }
    fetchData();
  }, []);

  const filtered = data.filter(d =>
    d.symbol.toLowerCase().includes(filter.toLowerCase())
  );

  return (
    <div className="dark min-h-screen p-8 bg-gray-900 text-white">
      <Navbar />
      <div className="w-full mx-auto grid gap-6">
        <div className="bg-gray-800 rounded-lg shadow-lg p-4">
        <div className="bg-gray-800 rounded-lg shadow-lg p-4">
  <input
    type="text"
    placeholder="Search symbol..."
    value={filter}
    onChange={e => setFilter(e.target.value)}
    className="w-full p-2 border border-gray-700 rounded bg-gray-700 text-white placeholder-gray-400"
  />
</div>
        </div>
        <div className="bg-gray-800 rounded-lg shadow-lg p-6 w-full" style={{ height: 500 }}>
          <NoSSRChart data={filtered} />
        </div>
      </div>
    </div>
  );
}