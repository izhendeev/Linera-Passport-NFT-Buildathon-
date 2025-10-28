"use client"

import { useMemo } from "react"
import { PassportGraphQLData } from "@/types/passport"

interface LeaderboardEntry {
  rank: number
  address: string
  score: number
}

interface LeaderboardProps {
  allPassports: PassportGraphQLData[]
  userScore: number
  userAddress: string
}

export function Leaderboard({ allPassports, userScore, userAddress }: LeaderboardProps) {
  // Generate real leaderboard from GraphQL data
  const leaderboard = useMemo(() => {
    // Sort passports by score (descending)
    const sorted = [...allPassports]
      .filter((p) => p.score !== null && p.score > 0)
      .sort((a, b) => (b.score || 0) - (a.score || 0))

    // Convert to leaderboard entries with ranks
    return sorted.map((passport, index) => ({
      rank: index + 1,
      address: passport.owner,
      score: passport.score || 0,
    }))
  }, [allPassports])

  // Calculate user's rank based on their score
  const userRank = leaderboard.findIndex((entry) => entry.address === userAddress) + 1 || leaderboard.length + 1

  return (
    <div className="flex h-fit w-72 flex-col gap-4 rounded-3xl border border-white/20 bg-white/10 p-6 shadow-2xl backdrop-blur-xl">
      {/* Your Position */}
      <div className="rounded-2xl border border-red-500/30 bg-gradient-to-br from-red-500/10 to-red-600/5 p-4 backdrop-blur-sm">
        <p className="text-center text-sm font-semibold text-stone-700">Your Place</p>
        <p className="text-center text-3xl font-bold text-red-600">#{userRank}</p>
      </div>

      {/* Leaderboard Title */}
      <div className="border-b border-stone-300/50 pb-2">
        <h3 className="text-center text-lg font-bold text-stone-800">Top Rankings</h3>
      </div>

      {/* Leaderboard Entries */}
      <div className="flex flex-col gap-2">
        {leaderboard.slice(0, 5).map((entry) => (
          <div
            key={entry.rank}
            className="flex items-center justify-between rounded-xl bg-white/30 px-3 py-2 backdrop-blur-sm transition-all hover:bg-white/40"
          >
            <div className="flex items-center gap-3">
              <span className="flex h-6 w-6 items-center justify-center rounded-full bg-stone-700 text-xs font-bold text-white">
                {entry.rank}
              </span>
              <span className="text-xs font-medium text-stone-600">
                {entry.address.slice(0, 6)}...{entry.address.slice(-4)}
              </span>
            </div>
            <span className="text-sm font-bold text-stone-800">{entry.score}</span>
          </div>
        ))}
      </div>
    </div>
  )
}
