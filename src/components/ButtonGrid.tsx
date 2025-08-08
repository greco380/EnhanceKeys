import React from 'react'

interface Snippet {
  id: string
  title: string
  text: string
  description: string
  hotkey: string
}

interface ButtonGridProps {
  snippets: Snippet[]
  onSnippetClick: (snippet: Snippet) => void
}

const ButtonGrid: React.FC<ButtonGridProps> = ({ snippets, onSnippetClick }) => {
  // Ensure we have exactly 9 snippets for the 3x3 grid
  const gridSnippets = Array.from({ length: 9 }, (_, index) => {
    return snippets.find(s => s.id === `button_${index + 1}`) || {
      id: `button_${index + 1}`,
      title: `Button ${index + 1}`,
      text: '',
      description: 'No snippet loaded',
      hotkey: `Alt+Shift+${index + 1}`
    }
  })

  return (
    <div className="grid grid-cols-3 gap-4 p-4">
      {gridSnippets.map((snippet) => (
        <button
          key={snippet.id}
          className="bg-white/90 backdrop-blur-sm border border-gray-200 rounded-lg p-4 
                     hover:bg-white hover:shadow-lg transition-all duration-200 
                     min-h-[120px] flex flex-col justify-between
                     focus:outline-none focus:ring-2 focus:ring-blue-500"
          onClick={() => onSnippetClick(snippet)}
          title={`${snippet.title} - ${snippet.hotkey}`}
        >
          <div className="text-center">
            <div className="font-semibold text-gray-900 text-sm mb-1">
              {snippet.title}
            </div>
            <div className="text-xs text-gray-600 mb-2">
              {snippet.description}
            </div>
          </div>
          <div className="flex justify-center">
            <span className="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded">
              {snippet.hotkey}
            </span>
          </div>
        </button>
      ))}
    </div>
  )
}

export default ButtonGrid 