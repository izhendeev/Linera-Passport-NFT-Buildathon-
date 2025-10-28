export interface PassportData {
  tokenId: string
  owner: string
  chainId: string
  score: number
  achievements: string[]
}

export interface PassportGraphQLData {
  tokenId: {
    id: number[] | string
  }
  owner: string
  ownerChain: string
  score: number | null
  achievements: string[] | null
}

export interface LeaderboardEntry {
  rank: number
  address: string
  score: number
}

// Helper to convert GraphQL passport to UI passport
export function convertPassportData(gqlPassport: PassportGraphQLData): PassportData {
  // Convert tokenId (array of bytes or hex string) to display format
  let tokenIdDisplay: string
  if (Array.isArray(gqlPassport.tokenId.id)) {
    // Take last 2 bytes and convert to a short readable number (0-65535)
    const bytes = gqlPassport.tokenId.id
    const lastTwoBytes = bytes.slice(-2)
    const shortId = (lastTwoBytes[0] || 0) * 256 + (lastTwoBytes[1] || 0)
    tokenIdDisplay = shortId.toString().padStart(4, "0")
  } else if (typeof gqlPassport.tokenId.id === "string") {
    // For hex string, take last 4 chars
    const lastChars = gqlPassport.tokenId.id.slice(-4)
    const shortId = parseInt(lastChars, 16)
    tokenIdDisplay = shortId.toString().padStart(4, "0")
  } else {
    tokenIdDisplay = "0000"
  }

  return {
    tokenId: tokenIdDisplay,
    owner: gqlPassport.owner,
    chainId: gqlPassport.ownerChain,
    score: gqlPassport.score || 0,
    achievements: gqlPassport.achievements || [],
  }
}
