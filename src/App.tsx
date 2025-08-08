import { useState, useEffect } from 'react'
import ButtonGrid from './components/ButtonGrid'
import { invoke } from '@tauri-apps/api/core'

interface Snippet {
  id: string
  title: string
  text: string
  description: string
  hotkey: string
}

interface Config {
  snippets: Snippet[]
  settings: {
    auto_start: boolean
    minimize_to_tray: boolean
    global_hotkeys: boolean
  }
  ui_settings: {
    always_on_top: boolean
    transparency: number
    position_x: string
    auto_start: boolean
  }
}

function App() {
  const [config, setConfig] = useState<Config | null>(null)
  const [showSettings, setShowSettings] = useState(false)

  useEffect(() => {
    loadConfiguration()
  }, [])

  const loadConfiguration = async () => {
    try {
      const loadedConfig = await invoke<Config>('load_config')
      setConfig(loadedConfig)
    } catch (error) {
      console.error('Failed to load configuration:', error)
    }
  }

  const handleSnippetClick = async (snippet: Snippet) => {
    try {
      await invoke('inject_text', { text: snippet.text })
    } catch (error) {
      console.error('Failed to inject text:', error)
    }
  }

  if (!config) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-100">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading EnhanceKeys...</p>
        </div>
      </div>
    )
  }

  return (
    <div 
      className="fixed inset-0 pointer-events-none"
      style={{ 
        background: `rgba(255, 255, 255, ${config.ui_settings.transparency})`,
        backdropFilter: 'blur(8px)'
      }}
    >
      <div className="absolute top-4 right-4 pointer-events-auto">
        <div className="bg-white/80 backdrop-blur-md rounded-lg shadow-lg border border-gray-200">
          <div className="p-4 border-b border-gray-200">
            <div className="flex justify-between items-center">
              <h1 className="text-lg font-bold text-gray-900">EnhanceKeys</h1>
              <button
                onClick={() => setShowSettings(!showSettings)}
                className="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 transition-colors"
              >
                ⚙️
              </button>
            </div>
          </div>
          
          {showSettings && (
            <div className="p-4 border-b border-gray-200">
              <p className="text-xs text-gray-600 mb-2">Settings panel (placeholder)</p>
              <button
                onClick={() => setShowSettings(false)}
                className="text-xs text-blue-600 hover:underline"
              >
                Close Settings
              </button>
            </div>
          )}

          <ButtonGrid snippets={config.snippets} onSnippetClick={handleSnippetClick} />
        </div>
      </div>
    </div>
  )
}

export default App 