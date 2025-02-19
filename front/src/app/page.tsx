"use client";

import { useEffect, useState } from "react";
import Image from "next/image";

interface Bird {
  name: string;
  image: string;
  description: string;
}

export default function Home() {
  const [bird, setBird] = useState<Bird | null>(null);

  // Fetch bird data from the API
  useEffect(() => {
    async function fetchBird() {
      try {
        const response = await fetch("http://127.0.0.1:8000/bird");
        if (response.ok) {
          const data = await response.json();
          console.log(1);
          setBird(data); // Store the fetched data in state
        } else {
          console.error("Failed to fetch bird data");
        }
      } catch (error) {
        console.error("Error fetching bird data:", error);
      }
    }

    fetchBird();
  }, []); // Empty dependency array ensures the fetch happens only once when the component mounts

  if (!bird) { //BIRD IS NULL!
    return (
      <div className="grid grid-rows-[40px_1fr_40px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-mono">
        <h1 className="text-5xl text-center">Bird of the Day</h1>
        <p>Loading bird data...</p>
      </div>
    );
  }

  return (
    <div style={{ fontFamily: 'Courier New, monospace' }} className="grid grid-rows-[40px_1fr_40px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-mono">
      <h1 className="text-5xl text-center">Bird of the Day</h1>
      <div className="flex items-center gap-8">
        <Image src={bird.image} alt={bird.name} width={300} height={300} />
        <p className="text-center">{bird.description}</p>
      </div>
      <a className="absolute top-3 right-40 size-16" href="https://www.github.com/kirtandusi">github.com/kirtandusi</a>
    </div>
  );
}
