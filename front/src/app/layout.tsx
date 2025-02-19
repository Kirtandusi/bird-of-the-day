import type { Metadata } from "next";
import "./globals.css";


export const metadata: Metadata = {
  title: "Bird of the Day",
  description: "Generates a new bird, along with a description and image, on refresh.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
       >{children}
      </body>
    </html>
  );
}
