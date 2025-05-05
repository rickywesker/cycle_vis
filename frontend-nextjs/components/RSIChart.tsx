import React, { useEffect, useState } from 'react';
import { Chart, ScatterController, CategoryScale, LinearScale, PointElement, Title, Tooltip, Legend } from 'chart.js';
import annotationPlugin from 'chartjs-plugin-annotation';
import DataLabelsPlugin from 'chartjs-plugin-datalabels';
import { Scatter } from 'react-chartjs-2';
import type { ChartOptions } from 'chart.js';
Chart.register(
  ScatterController,
  CategoryScale,
  LinearScale,
  PointElement,
  Title,
  Tooltip,
  Legend,
  annotationPlugin,
  DataLabelsPlugin
);
Chart.defaults.color = '#ffffff';
interface IndicatorResult { symbol: string; value: number; category: string; }
interface Props { data: IndicatorResult[]; }

export default function RSIChart({ data }: Props) {
  const [btcImg, setBtcImg] = useState<HTMLImageElement | null>(null);
  const [ethImg, setEthImg] = useState<HTMLImageElement | null>(null);

  useEffect(() => {
    const btc = new Image(16, 16);
    btc.src = '/btc.png';
    const eth = new Image(16, 16);
    eth.src = '/eth.svg';
    setBtcImg(btc);
    setEthImg(eth);
  }, []);

  const points = data.map(d => ({ x: d.symbol, y: d.value }));
  const pointStyles = data.map(d => {
    if (d.symbol === 'BTCUSDT' && btcImg) return btcImg;
    if (d.symbol === 'ETHUSDT' && ethImg) return ethImg;
    return 'circle';
  });
  const pointRadius = data.map(d => (['BTCUSDT','ETHUSDT'].includes(d.symbol) ? 16 : 3));
  const pointColor = data.map(d => (['BTCUSDT','ETHUSDT'].includes(d.symbol) ? 'transparent' : d.value < 40 ? 'green' : d.value > 70 ? 'red' : 'gray'));

  const chartData = {
    datasets: [{
      label: 'RSI',
      data: points,
      pointStyle: pointStyles as any,
      pointRadius,
      pointBackgroundColor: pointColor,
      showLine: false
    }]
  };

  const options: ChartOptions<'scatter'> = {
    responsive: true,
    layout: { padding: { left: 20, right: 20 } },
    plugins: {
      legend: { position: 'top' as const, labels: { color: '#ffffff' } },
      title: { display: true, text: 'Relative Strength Index', color: '#ffffff',font: { size: 20, weight: 'bold' as const } },
      tooltip: {
        callbacks: {
          label: (ctx) => {
            const { x, y } = ctx.raw as { x: string; y: number };  // 👈 明确断言类型
            const sym = x.replace(/USDT$/, '');
            return `${sym}: ${y.toFixed(2)}`;
          }
        }
      },
      datalabels: {
        align: 'top',
        color: '#ffffff',
        formatter: (value) => {
          const v = value as { x: string; y: number };
          if (v.y < 40 || v.y > 70) return v.x.replace(/USDT$/, '');
          return null;
        }
      },
      annotation: {
        annotations: {
          lowZone: { type: 'box' as const, yMin: 0, yMax: 40, backgroundColor: 'rgba(0,128,0,0.2)' },
          midZone: { type: 'box' as const, yMin: 40, yMax: 70, backgroundColor: 'rgba(128,128,128,0.2)' },
          highZone:{ type:'box' as const, yMin: 70, yMax: 100, backgroundColor:'rgba(255,0,0,0.2)' }
        }
      }
    },
    scales: {
      x: { display: false,type: 'category', labels: data.map(d => d.symbol)},
      y: { beginAtZero: true, max: 100, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255,255,255,0.1)' } }
    },
    onClick: (evt: any, elements: any[]) => {
      if (elements.length > 0) {
        const idx = elements[0].index;
        const item = data[idx];
        alert(`${item.symbol}\nRSI: ${item.value.toFixed(2)}\nCategory: ${item.category}`);
      }
    }
  };

  return <Scatter data={chartData} options={options} />;
}

