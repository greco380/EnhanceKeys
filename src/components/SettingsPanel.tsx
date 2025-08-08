import React, { useState } from 'react'
import Button from './Button'

interface Snippet {
  id: string
  name: string
  text: string
  hotkey?: string
}

interface SettingsPanelProps {
  snippets: Snippet[]
  onAddSnippet: (snippet: Omit<Snippet, 'id'>) => void
  onDeleteSnippet: (id: string) => void
  onClose: () => void
}

const SettingsPanel: React.FC<SettingsPanelProps> = ({
  snippets,
  onAddSnippet,
  onDeleteSnippet,
  onClose
}) => {
  const [newSnippet, setNewSnippet] = useState({
    name: '',
    text: '',
    hotkey: ''
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (newSnippet.name.trim() && newSnippet.text.trim()) {
      onAddSnippet({
        name: newSnippet.name.trim(),
        text: newSnippet.text.trim(),
        hotkey: newSnippet.hotkey.trim() || undefined
      })
      setNewSnippet({ name: '', text: '', hotkey: '' })
    }
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
        <div className="p-6">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-gray-900">Settings</h2>
            <Button
              variant="secondary"
              size="sm"
              onClick={onClose}
            >
              Close
            </Button>
          </div>

          {/* Add New Snippet */}
          <div className="card p-6 mb-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Add New Snippet</h3>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                  Name
                </label>
                <input
                  type="text"
                  id="name"
                  value={newSnippet.name}
                  onChange={(e) => setNewSnippet({ ...newSnippet, name: e.target.value })}
                  className="input-field"
                  placeholder="Enter snippet name"
                  required
                />
              </div>
              
              <div>
                <label htmlFor="text" className="block text-sm font-medium text-gray-700 mb-1">
                  Text
                </label>
                <textarea
                  id="text"
                  value={newSnippet.text}
                  onChange={(e) => setNewSnippet({ ...newSnippet, text: e.target.value })}
                  className="input-field min-h-[100px] resize-y"
                  placeholder="Enter your snippet text"
                  required
                />
              </div>
              
              <div>
                <label htmlFor="hotkey" className="block text-sm font-medium text-gray-700 mb-1">
                  Hotkey (Optional)
                </label>
                <input
                  type="text"
                  id="hotkey"
                  value={newSnippet.hotkey}
                  onChange={(e) => setNewSnippet({ ...newSnippet, hotkey: e.target.value })}
                  className="input-field"
                  placeholder="e.g., Ctrl+Shift+G"
                />
              </div>
              
              <div className="flex justify-end">
                <Button type="submit" disabled={!newSnippet.name.trim() || !newSnippet.text.trim()}>
                  Add Snippet
                </Button>
              </div>
            </form>
          </div>

          {/* Existing Snippets */}
          <div className="card p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Manage Snippets</h3>
            {snippets.length === 0 ? (
              <p className="text-gray-500 text-center py-4">No snippets created yet</p>
            ) : (
              <div className="space-y-3">
                {snippets.map((snippet) => (
                  <div key={snippet.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <span className="font-medium text-gray-900">{snippet.name}</span>
                        {snippet.hotkey && (
                          <span className="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded">
                            {snippet.hotkey}
                          </span>
                        )}
                      </div>
                      <p className="text-sm text-gray-600 truncate">{snippet.text}</p>
                    </div>
                    <Button
                      variant="danger"
                      size="sm"
                      onClick={() => onDeleteSnippet(snippet.id)}
                    >
                      Delete
                    </Button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

export default SettingsPanel 