"use client"

import { useState, useEffect } from "react"
import { ApolloProvider, ApolloClient, InMemoryCache, gql, useQuery } from "@apollo/client"
import { WalletButton } from "@/components/wallet-button"
import { PassportCard } from "@/components/passport-card"
import { Leaderboard } from "@/components/leaderboard"
import { FloatingParticles } from "@/components/floating-particles"
import { Button } from "@/components/ui/button"
import { useToast } from "@/hooks/use-toast"

const GRAPHQL_ENDPOINT = process.env.NEXT_PUBLIC_GRAPHQL_ENDPOINT || ""

const apolloClient = new ApolloClient({
  uri: GRAPHQL_ENDPOINT,
  cache: new InMemoryCache(),
})

const GET_ALL_PASSPORTS = gql`
  query GetAllPassports {
    allPassports {
      tokenId { id }
      owner
      ownerChain
      score
      achievements
    }
  }
`

function PassportApp() {
  const [walletAddress, setWalletAddress] = useState<string | null>(null)
  const { data, loading, error, refetch } = useQuery(GET_ALL_PASSPORTS, {
    pollInterval: 5000,
  })
  const { toast } = useToast()

  useEffect(() => {
    const saved = localStorage.getItem("linera_wallet")
    if (saved) {
      setWalletAddress(saved)
    }
  }, [])

  const handleConnect = (address: string) => {
    setWalletAddress(address)
  }

  const handleDisconnect = () => {
    setWalletAddress(null)
    localStorage.removeItem("linera_wallet")
    localStorage.removeItem("linera_wallet_type")
  }

  const userPassport = data?.allPassports?.find(
    (p: any) => p.owner.toLowerCase() === walletAddress?.toLowerCase()
  )

  const allPassports = data?.allPassports || []
  const userScore = userPassport?.score || 0

  return (
    <div className="relative min-h-screen w-full overflow-hidden bg-gradient-to-br from-stone-300 via-stone-200 to-stone-100">
      <FloatingParticles />
      
      <div className="relative z-10 flex min-h-screen flex-col">
        <header className="flex items-center justify-between p-6">
          <h1 className="text-3xl font-bold tracking-tight text-stone-800">
            Linera Passport NFT
          </h1>
          <WalletButton
            walletAddress={walletAddress}
            onConnect={handleConnect}
            onDisconnect={handleDisconnect}
          />
        </header>

        <main className="flex flex-1 gap-8 p-6">
          <div className="flex-1">
            {!walletAddress ? (
              <div className="flex h-96 items-center justify-center rounded-2xl border border-stone-300 bg-white/30 backdrop-blur-sm">
                <p className="text-lg text-stone-600">Connect your Linera wallet to view passport</p>
              </div>
            ) : loading ? (
              <div className="flex h-96 items-center justify-center">
                <p className="text-lg text-stone-600">Loading passport...</p>
              </div>
            ) : error ? (
              <div className="flex h-96 flex-col items-center justify-center gap-2">
                <p className="text-lg text-red-600">Error: {error.message}</p>
                <p className="text-sm text-stone-500">Make sure linera service is running on port 8080</p>
              </div>
            ) : userPassport ? (
              <PassportCard passport={userPassport} onUpdate={refetch} />
            ) : (
              <div className="flex h-96 flex-col items-center justify-center gap-4 rounded-2xl border border-stone-300 bg-white/30 backdrop-blur-sm">
                <p className="text-lg text-stone-600">No passport found for this wallet</p>
                <Button
            ) : (
              <div className="flex h-96 flex-col items-center justify-center gap-4 rounded-2xl border border-stone-300 bg-white/30 backdrop-blur-sm">
                <p className="text-lg text-stone-600">No passport found for this wallet</p>
                <Button
                  onClick={async () => {
                    try {
                      const response = await fetch('http://localhost:8082/mint', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ owner: walletAddress })
                      })
                      if (!response.ok) throw new Error('Mint failed')
                      toast({ title: "Passport minted!", description: "Refreshing..." })
                      setTimeout(() => refetch(), 2000)
                    } catch (error: any) {
                      toast({ title: "Mint failed", description: error.message + " (Ensure mint API runs on port 8082)", variant: "destructive" })
                    }
                  }}
                  className="bg-gradient-to-br from-red-600 to-red-800 text-white"
                >
                  Mint Passport NFT
                </Button>
              </div>
            )}
              </div>
            )}
          </div>

          <aside className="w-80">
            <Leaderboard 
              allPassports={allPassports}
              userScore={userScore}
              userAddress={walletAddress || ""}
            />
          </aside>
        </main>
      </div>
    </div>
  )
}

export default function Home() {
  return (
    <ApolloProvider client={apolloClient}>
      <PassportApp />
    </ApolloProvider>
  )
}
