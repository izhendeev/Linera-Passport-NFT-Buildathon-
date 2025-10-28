import type { Metadata } from "next"
import { Michroma } from "next/font/google"
import "./globals.css"
import { Toaster } from "@/components/ui/toaster"

const michroma = Michroma({
  weight: "400",
  subsets: ["latin"],
})

export const metadata: Metadata = {
  title: "Linera Passport NFT",
  description: "AI-powered reputation passports on Linera blockchain",
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body className={michroma.className}>
        {children}
        <Toaster />
      </body>
    </html>
  )
}
