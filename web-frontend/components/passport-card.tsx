"use client"

import { Button } from "@/components/ui/button"
import { Sparkles } from "lucide-react"

interface Passport {
  tokenId: { id: number[] }
  owner: string
  ownerChain: string
  score: number
  achievements: string[]
}

interface PassportCardProps {
  passport: Passport
  onUpdate: () => void
}

export function PassportCard({ passport, onUpdate }: PassportCardProps) {
  const tokenIdHex = passport.tokenId.id
    .map(n => n.toString(16).padStart(2, '0'))
    .join('')
    .slice(0, 16)

  return (
    <div className="group relative h-[400px] w-full max-w-md perspective-1000">
      <div className="relative h-full w-full rounded-3xl border border-white/20 bg-gradient-to-br from-red-500/10 via-blue-500/10 to-purple-500/10 p-8 shadow-2xl backdrop-blur-xl transition-all duration-500 hover:shadow-red-500/20">
        
        {/* Header */}
        <div className="mb-6 flex items-center justify-between border-b border-white/20 pb-4">
          <h2 className="text-2xl font-bold text-stone-800">Passport NFT</h2>
          <Sparkles className="h-6 w-6 text-red-500" />
        </div>

        {/* Token ID */}
        <div className="mb-4">
          <p className="text-xs font-light uppercase tracking-widest text-stone-500">Token ID</p>
          <p className="font-mono text-sm font-bold text-stone-700">{tokenIdHex}...</p>
        </div>

        {/* Owner */}
        <div className="mb-4">
          <p className="text-xs font-light uppercase tracking-widest text-stone-500">Owner</p>
          <p className="font-mono text-sm font-bold text-stone-700">
            {passport.owner.slice(0, 10)}...{passport.owner.slice(-8)}
          </p>
        </div>

        {/* Score */}
        <div className="mb-6">
          <p className="text-xs font-light uppercase tracking-widest text-stone-500">Reputation Score</p>
          <p className="text-5xl font-bold text-red-600">{passport.score}</p>
        </div>

        {/* Achievements */}
        <div className="mb-6">
          <p className="mb-2 text-xs font-light uppercase tracking-widest text-stone-500">
            Achievements ({passport.achievements?.length || 0})
          </p>
          <div className="max-h-24 overflow-y-auto rounded-lg bg-black/5 p-2">
            {passport.achievements && passport.achievements.length > 0 ? (
              <div className="space-y-1">
                {passport.achievements.map((ach, idx) => (
                  <p key={idx} className="text-xs text-stone-600">• {ach}</p>
                ))}
              </div>
            ) : (
              <p className="text-xs text-stone-400">No achievements yet</p>
            )}
          </div>
        </div>

        {/* Update Button */}
        <Button
          onClick={onUpdate}
          className="w-full bg-gradient-to-br from-purple-600 to-indigo-700 font-bold text-white shadow-lg transition-all hover:scale-105"
        >
          <Sparkles className="mr-2 h-4 w-4" />
          Refresh Data
        </Button>
      </div>
    </div>
  )
}
