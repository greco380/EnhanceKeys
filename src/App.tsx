import React, { useState } from 'react'
import ButtonGrid from './components/ButtonGrid'
import SettingsPanel from './components/SettingsPanel'
import { invoke } from '@tauri-apps/api/tauri'

interface Snippet {
  id: string
  name: string
  text: string
  hotkey?: string
}

function App() {
  const [snippets, setSnippets] = useState<Snippet[]>([
    {
      id: '1',
      name: 'Greeting',
      text: 'Hello! How can I help you today?',
      hotkey: 'Ctrl+Shift+G'
    },
    {
      id: '2',
      name: 'Code Review',
      text: 'Please review this code and provide feedback on improvements, best practices, and potential issues.',
      hotkey: 'Ctrl+Shift+R'
    },
    {
      id: '3',
      name: 'Bug Report',
      text: 'I\'m experiencing an issue with the following behavior: [describe the problem]. Expected behavior: [describe what should happen].',
      hotkey: 'Ctrl+Shift+B'
    }
  ])

  const [showSettings, setShowSettings] = useState(false)

  const handleSnippetClick = async (snippet: Snippet) => {
    try {
      await invoke('inject_text', { text: snippet.text })
    } catch (error) {
      console.error('Failed to inject text:', error)
    }
  }

  const handleAddSnippet = (newSnippet: Omit<Snippet, 'id'>) => {
    const snippet: Snippet = {
      ...newSnippet,
      id: Date.now().toString()
    }
    setSnippets([...snippets, snippet])
  }

  const handleDeleteSnippet = (id: string) => {
    setSnippets(snippets.filter(s => s.id !== id))
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-6xl mx-auto p-6">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">EnhanceKeys</h1>
            <p className="text-gray-600 mt-2">Quick text snippets for AI chat windows</p>
          </div>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
          >
            {showSettings ? 'Hide Settings' : 'Settings'}
          </button>
        </div>

        {/* Settings Panel */}
        {showSettings && (
          <SettingsPanel
            snippets={snippets}
            onAddSnippet={handleAddSnippet}
            onDeleteSnippet={handleDeleteSnippet}
            onClose={() => setShowSettings(false)}
          />
        )}

        {/* Main Content */}
        <div className="space-y-6">
          <div className="bg-white rounded-lg shadow-sm p-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">Your Snippets</h2>
            <ButtonGrid snippets={snippets} onSnippetClick={handleSnippetClick} />
          </div>
        </div>
      </div>
    </div>
  )
}

export default App 