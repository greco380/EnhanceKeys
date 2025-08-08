import React from 'react'
import Button from './Button'

interface Snippet {
  id: string
  name: string
  text: string
  hotkey?: string
}

interface ButtonGridProps {
  snippets: Snippet[]
  onSnippetClick: (snippet: Snippet) => void
}

const ButtonGrid: React.FC<ButtonGridProps> = ({ snippets, onSnippetClick }) => {
  if (snippets.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="text-gray-500 text-lg mb-4">No snippets yet</div>
        <p className="text-gray-400">Create your first snippet in the settings panel</p>
      </div>
    )
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {snippets.map((snippet) => (
        <div
          key={snippet.id}
          className="card p-4 hover:shadow-md transition-shadow cursor-pointer group"
          onClick={() => onSnippetClick(snippet)}
        >
          <div className="flex justify-between items-start mb-3">
            <h3 className="font-semibold text-gray-900 group-hover:text-primary-600 transition-colors">
              {snippet.name}
            </h3>
            {snippet.hotkey && (
              <span className="text-xs bg-gray-100 text-gray-600 px-2 py-1 rounded">
                {snippet.hotkey}
              </span>
            )}
          </div>
          <p className="text-gray-600 text-sm line-clamp-3 mb-4">
            {snippet.text}
          </p>
          <div className="flex justify-between items-center">
            <Button
              variant="primary"
              size="sm"
              onClick={(e) => {
                e.stopPropagation()
                onSnippetClick(snippet)
              }}
            >
              Use Snippet
            </Button>
            <span className="text-xs text-gray-400">
              {snippet.text.length} chars
            </span>
          </div>
        </div>
      ))}
    </div>
  )
}

export default ButtonGrid 