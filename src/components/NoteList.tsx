import React from 'react';

interface Note {
  id: number;
  title: string;
  content: string;
  knowledge_type: string;
  tags: string[];
}

interface NoteListProps {
  notes: Note[];
  selectedNoteId: number | null;
  onSelect: (note: Note) => void;
  searchTerm?: string;
}

const highlightText = (text: string, search: string): React.ReactNode => {
  if (!search.trim()) return text;
  
  const regex = new RegExp(`(${search.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
  const parts = text.split(regex);
  
  return parts.map((part, i) => 
    regex.test(part) ? (
      <span key={i} className="highlight">{part}</span>
    ) : part
  );
};

export const NoteList: React.FC<NoteListProps> = ({ 
  notes, 
  selectedNoteId, 
  onSelect,
  searchTerm = ''
}) => {
  if (notes.length === 0) {
    return (
      <div className="empty-state">
        <p>No notes found</p>
        <button onClick={() => document.dispatchEvent(new CustomEvent('quickAdd'))}>
          Press Ctrl+K to add one!
        </button>
      </div>
    );
  }

  return (
    <div className="note-list">
      {notes.map((note) => (
        <div
          key={note.id}
          className={`note-item ${selectedNoteId === note.id ? 'selected' : ''}`}
          onClick={() => onSelect(note)}
        >
          <h4>{highlightText(note.title, searchTerm)}</h4>
          
          {note.tags && note.tags.length > 0 && (
            <div className="tags">
              {note.tags.map((tag, i) => (
                <span key={i} className="tag">{`#${tag}`}</span>
              ))}
            </div>
          )}
          
          <p className="preview">{highlightText(note.content.substring(0, 80), searchTerm)}...</p>
        </div>
      ))}
    </div>
  );
};
