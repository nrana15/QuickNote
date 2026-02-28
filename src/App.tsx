import React, { useState, useEffect } from 'react';
import { SearchBar } from './components/SearchBar';
import { QuickAddModal } from './components/QuickAddModal';
import { NoteList } from './components/NoteList';
import { ReviewMode } from './components/ReviewMode';

interface Note {
  id: number;
  title: string;
  content: string;
  knowledge_type: string;
  tags: string[];
}

declare global {
  interface Window {
    __TAURI__: {
      invoke: (cmd: string, args?: any) => Promise<any>;
    };
  }
}

const App = () => {
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedNoteId, setSelectedNoteId] = useState<number | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [isQuickAddOpen, setIsQuickAddOpen] = useState(false);
  const [isReviewModeOpen, setIsReviewModeOpen] = useState(false);

  useEffect(() => {
    // Load initial notes from backend
    loadNotes();
    
    // Register keyboard shortcuts
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsQuickAddOpen(true);
      }
      
      if ((e.metaKey || e.ctrlKey) && e.key === 'e') {
        e.preventDefault();
        handleExport();
      }

      if ((e.metaKey || e.ctrlKey) && e.key === 'r') {
        e.preventDefault();
        setIsReviewModeOpen(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    
    // Listen for quick add event from SearchBar
    const handleQuickAdd = () => setIsQuickAddOpen(true);
    document.addEventListener('quickAdd' as any, handleQuickAdd);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('quickAdd' as any, handleQuickAdd);
    };
  }, []);

  const loadNotes = async () => {
    try {
      const result = await window.__TAURI__.invoke('search_notes', { query: '' });
      setNotes(result || []);
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  };

  const handleSearch = async (query: string) => {
    setSearchTerm(query);
    
    try {
      if (!query.trim()) {
        await loadNotes();
        return;
      }

      const result = await window.__TAURI__.invoke('search_notes', { query });
      setNotes(result || []);
    } catch (error) {
      console.error('Search failed:', error);
    }
  };

  const handleNoteSelect = (note: Note) => {
    setSelectedNoteId(note.id);
  };

  const handleSaveNote = async (title: string, content: string) => {
    try {
      await window.__TAURI__.invoke('add_note', { title, content });
      await loadNotes(); // Refresh list
      
      // Auto-focus search after adding
      document.querySelector('.search-bar')?.focus();
    } catch (error) {
      console.error('Failed to save note:', error);
      alert('Failed to save note. Please try again.');
    }
  };

  const handleExport = async () => {
    try {
      await window.__TAURI__.invoke('export_vault', {});
      alert('Vault exported successfully!');
    } catch (error) {
      console.error('Export failed:', error);
      alert('Failed to export vault. Please check your system permissions.');
    }
  };

  const handleCardComplete = async (cardId: number, rating: 'again' | 'hard' | 'good' | 'easy') => {
    try {
      await window.__TAURI__.invoke('rate_review_card', { cardId, rating });
      // Card will be removed from list automatically on next load
    } catch (error) {
      console.error('Failed to rate card:', error);
    }
  };

  const selectedNote = notes.find(note => note.id === selectedNoteId);

  return (
    <div className="app-container">
      {/* Sidebar */}
      <aside className="sidebar">
        <h3>QuickNote</h3>
        
        <button 
          onClick={() => setIsQuickAddOpen(true)}
          className="quick-add-btn"
        >
          Quick Add (Ctrl+K)
        </button>

        <div style={{ marginTop: '1rem' }}>
          <button 
            onClick={handleExport}
            className="btn-export"
          >
            Export Vault (Ctrl+E)
          </button>
        </div>

        <button 
          onClick={() => setIsReviewModeOpen(true)}
          className="review-btn"
        >
          Review Mode (Ctrl+R)
        </button>

        <div style={{ marginTop: '2rem', fontSize: '0.85em', color: '#64748b' }}>
          <p><strong>Status:</strong> {notes.length} notes</p>
          <p><strong>Mode:</strong> Portable</p>
        </div>
      </aside>

      {/* Main Content */}
      <main className="main-content">
        <SearchBar onSearch={handleSearch} />
        
        <NoteList 
          notes={notes.filter(note => 
            note.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
            note.content.toLowerCase().includes(searchTerm.toLowerCase())
          )}
          selectedNoteId={selectedNoteId}
          onSelect={handleNoteSelect}
          searchTerm={searchTerm}
        />

        {/* Note Detail View */}
        {selectedNote && (
          <div className="note-detail">
            <h2>{selectedNote.title}</h2>
            
            <div className="meta-info">
              <span className={`type-badge ${selectedNote.knowledge_type.toLowerCase()}`}>
                {selectedNote.knowledge_type}
              </span>
              
              {selectedNote.tags && selectedNote.tags.length > 0 && (
                <div className="tags-display">
                  {selectedNote.tags.map((tag, i) => (
                    <span key={i} className="detail-tag">{`#${tag}`}</span>
                  ))}
                </div>
              )}
            </div>
            
            <div className="note-content">
              <pre>{selectedNote.content}</pre>
            </div>
          </div>
        )}
      </main>

      {/* Quick Add Modal */}
      <QuickAddModal 
        isOpen={isQuickAddOpen}
        onClose={() => setIsQuickAddOpen(false)}
        onSave={handleSaveNote}
      />

      {/* Review Mode */}
      <ReviewMode 
        isOpen={isReviewModeOpen}
        onClose={() => setIsReviewModeOpen(false)}
        onCardComplete={handleCardComplete}
      />
    </div>
  );
};

export default App;
