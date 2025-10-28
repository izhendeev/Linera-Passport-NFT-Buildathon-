/**
 * Demo wallet helper used by the mock UI wallet button.
 * Relies purely on `ethers` library (no Linera SDK involvement).
 */

import { Wallet } from 'ethers'

export class DemoWallet {
  private static STORAGE_KEY = 'linera_demo_private_key'

  private static ensureClientSide() {
    if (typeof window === 'undefined') {
      throw new Error('DemoWallet helper is browser-only')
    }
  }

  /**
   * Get or create a demo wallet private key.
   * Persists key material in localStorage for repeatable demos.
   */
  static getOrCreatePrivateKey(): string {
    this.ensureClientSide()

    const existingKey = localStorage.getItem(this.STORAGE_KEY)
    if (existingKey) {
      console.log('🔑 Using existing demo private key')
      return existingKey
    }

    const wallet = Wallet.createRandom()
    const privateKey = wallet.privateKey
    localStorage.setItem(this.STORAGE_KEY, privateKey)

    console.log('🎲 Generated new demo private key')
    console.log('   Address:', wallet.address)

    return privateKey
  }

  /**
   * Derive checksum address from the stored private key.
   */
  static getAddress(): string {
    this.ensureClientSide()
    const privateKey = this.getOrCreatePrivateKey()
    return new Wallet(privateKey).address
  }

  /**
   * Utility used in tests/debug panels.
   */
  static clear(): void {
    this.ensureClientSide()
    localStorage.removeItem(this.STORAGE_KEY)
    console.log('🗑️ Demo wallet cleared')
  }
}
