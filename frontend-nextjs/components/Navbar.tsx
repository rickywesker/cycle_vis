import Link from 'next/link';

export default function Navbar() {
  return (
      <nav className="bg-gray-800 px-6 py-4 rounded-md mb-6 shadow-lg">
          <div className="flex items-center gap-8">
          <h1 className="text-teal-400 text-xl font-extrabold tracking-widest select-none hover:text-teal-300 transition">
        CYCLESTUDY <span className="text-white">VIS</span>
      </h1>
          <ul className="flex gap-6 text-sm font-medium">
        <li>
          <Link href="/" className="text-white hover:text-gray-300">RSI</Link>
        </li>
        
        
              </ul>
          </div>
    </nav>
  );
}