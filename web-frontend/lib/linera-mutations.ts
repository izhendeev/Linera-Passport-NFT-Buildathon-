/**
 * Linera Passport NFT Mutations
 * These functions handle actual blockchain transactions using Linera Client SDK
 */

export interface TokenIdInput {
  id: number[]
}

export interface MintPassportParams {
  tokenId: TokenIdInput
  metadataUri: string
  imageUri: string
  contentHash: string
}

export interface UpdateAchievementsParams {
  tokenId: TokenIdInput
  newAchievements: string[]
  scoreIncrease: number
}

/**
 * Generate a random Token ID (16 bytes)
 */
export function generateTokenId(): TokenIdInput {
  const bytes = Array.from(crypto.getRandomValues(new Uint8Array(16)))
  return { id: bytes }
}

/**
 * Generate a random content hash (32 bytes hex string)
 */
export function generateContentHash(): string {
  const bytes = Array.from(crypto.getRandomValues(new Uint8Array(32)))
  return '0x' + bytes.map(b => b.toString(16).padStart(2, '0')).join('')
}

/**
 * Mint a new Passport NFT
 * This sends a signed transaction to the blockchain
 */
export async function mintPassport(
  client: any, // Will be typed properly when SDK is loaded
  params: MintPassportParams
): Promise<void> {
  const mutation = `
    mutation MintPassport(
      $tokenId: TokenIdInput!
      $metadataUri: String!
      $imageUri: String!
      $contentHash: String!
    ) {
      mint(
        tokenId: $tokenId
        metadataUri: $metadataUri
        imageUri: $imageUri
        contentHash: $contentHash
      )
    }
  `

  const variables = {
    tokenId: params.tokenId,
    metadataUri: params.metadataUri,
    imageUri: params.imageUri,
    contentHash: params.contentHash,
  }

  // Execute mutation through Linera Client SDK
  // The SDK will sign the transaction with the user's private key
  const result = await client.mutate(JSON.stringify({
    query: mutation,
    variables,
  }))

  return JSON.parse(result)
}

/**
 * Update achievements and score for a Passport
 * This is called by the Oracle Agent
 */
export async function updateAchievements(
  client: any,
  params: UpdateAchievementsParams
): Promise<void> {
  const mutation = `
    mutation UpdateAchievements(
      $tokenId: TokenIdInput!
      $newAchievements: [String!]!
      $scoreIncrease: Int!
    ) {
      updateAchievements(
        tokenId: $tokenId
        newAchievements: $newAchievements
        scoreIncrease: $scoreIncrease
      )
    }
  `

  const variables = {
    tokenId: params.tokenId,
    newAchievements: params.newAchievements,
    scoreIncrease: params.scoreIncrease,
  }

  const result = await client.mutate(JSON.stringify({
    query: mutation,
    variables,
  }))

  return JSON.parse(result)
}

/**
 * Request AI score analysis from Oracle Agent
 * This triggers the Oracle to analyze reputation and update the passport
 */
export async function requestAIScore(
  chainId: string,
  applicationId: string,
  owner: string
): Promise<void> {
  // Call Oracle Agent HTTP endpoint
  const oracleUrl = 'http://localhost:8081/analyze'

  const response = await fetch(oracleUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      chain_id: chainId,
      application_id: applicationId,
      owner: owner,
    }),
  })

  if (!response.ok) {
    throw new Error(`Oracle request failed: ${response.statusText}`)
  }

  return await response.json()
}
