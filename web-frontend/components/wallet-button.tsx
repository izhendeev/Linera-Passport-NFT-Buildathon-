"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Wallet, LogOut } from "lucide-react"
import { useToast } from "@/hooks/use-toast"

interface WalletButtonProps {
  walletAddress: string | null
  onConnect: (address: string) => void
  onDisconnect: () => void
}

// Linera wallet address from ~/.config/linera/wallet.json
const LINERA_WALLET_ADDRESS = "0xa2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74"

export function WalletButton({ walletAddress, onConnect, onDisconnect }: WalletButtonProps) {
  const [isLoading, setIsLoading] = useState(false)
  const { toast } = useToast()

  const handleConnectLineraWallet = async () => {
    setIsLoading(true)
    try {
      console.log("🔗 Connecting to Linera wallet...")

      // Use Linera wallet address
      const address = LINERA_WALLET_ADDRESS
      console.log("✅ Linera wallet connected:", address)

      onConnect(address)
      localStorage.setItem("linera_wallet", address)
      localStorage.setItem("linera_wallet_type", "linera")

      toast({
        title: "Linera wallet connected!",
        description: "Address: " + address.slice(0, 10) + "..." + address.slice(-8),
      })
    } catch (error: any) {
      console.error("❌ Failed to connect Linera wallet:", error)
      toast({
        title: "Error",
        description: error.message || "Failed to connect Linera wallet",
        variant: "destructive",
      })
    } finally {
      setIsLoading(false)
    }
  }

  if (walletAddress) {
    return (
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-2 rounded-md border border-stone-300 bg-white/50 px-3 py-2 backdrop-blur-sm">
          <Wallet className="h-4 w-4" />
          <span className="font-mono text-sm font-bold">
            {walletAddress.slice(0, 6)}...{walletAddress.slice(-4)}
          </span>
        </div>
        <Button
          variant="outline"
          size="sm"
          onClick={onDisconnect}
          className="gap-2 border-stone-300 bg-white/50 backdrop-blur-sm"
        >
          <LogOut className="h-4 w-4" />
          Disconnect
        </Button>
      </div>
    )
  }

  return (
    <Button
      onClick={handleConnectLineraWallet}
      disabled={isLoading}
      className="gap-2 bg-gradient-to-br from-stone-700 to-stone-900 font-bold text-white shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-stone-900/50 disabled:opacity-50"
    >
      <Wallet className="h-4 w-4" />
      {isLoading ? "Connecting..." : "Connect Linera Wallet"}
    </Button>
  )
}
