import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface Snippet {
  id: string
  name: string
  text: string
  hotkey?: string
}

interface Config {
  snippets: Snippet[]
  settings: {
    autoStart: boolean
    minimizeToTray: boolean
    globalHotkeys: boolean
  }
}

export const useConfig = () => {
  const [config, setConfig] = useState<Config>({
    snippets: [],
    settings: {
      autoStart: false,
      minimizeToTray: true,
      globalHotkeys: true
    }
  })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const loadConfig = async () => {
    try {
      setLoading(true)
      const loadedConfig = await invoke<Config>('load_config')
      setConfig(loadedConfig)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load configuration')
    } finally {
      setLoading(false)
    }
  }

  const saveConfig = async (newConfig: Config) => {
    try {
      await invoke('save_config', { config: newConfig })
      setConfig(newConfig)
      setError(null)
      return true
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save configuration')
      return false
    }
  }

  const addSnippet = async (snippet: Omit<Snippet, 'id'>) => {
    const newSnippet: Snippet = {
      ...snippet,
      id: Date.now().toString()
    }
    const newConfig = {
      ...config,
      snippets: [...config.snippets, newSnippet]
    }
    const success = await saveConfig(newConfig)
    return success ? newSnippet : null
  }

  const updateSnippet = async (id: string, updates: Partial<Snippet>) => {
    const newConfig = {
      ...config,
      snippets: config.snippets.map(snippet =>
        snippet.id === id ? { ...snippet, ...updates } : snippet
      )
    }
    return await saveConfig(newConfig)
  }

  const deleteSnippet = async (id: string) => {
    const newConfig = {
      ...config,
      snippets: config.snippets.filter(snippet => snippet.id !== id)
    }
    return await saveConfig(newConfig)
  }

  const updateSettings = async (settings: Partial<Config['settings']>) => {
    const newConfig = {
      ...config,
      settings: { ...config.settings, ...settings }
    }
    return await saveConfig(newConfig)
  }

  useEffect(() => {
    loadConfig()
  }, [])

  return {
    config,
    loading,
    error,
    loadConfig,
    saveConfig,
    addSnippet,
    updateSnippet,
    deleteSnippet,
    updateSettings
  }
} 