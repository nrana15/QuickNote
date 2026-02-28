import React, { useState, useEffect } from 'react';

interface QuickAddModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (title: string, content: string) => void;
}

export const QuickAddModal: React.FC<QuickAddModalProps> = ({ 
  isOpen, 
  onClose, 
  onSave 
}) => {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const titleRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => titleRef.current?.focus(), 100);
    }
  }, [isOpen]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;
      
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        onClose();
      } else if (e.key === 'Escape') {
        onClose();
      } else if (e.key === 'Enter' && e.ctrlKey) {
        e.preventDefault();
        handleSave();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  const handleSave = () => {
    if (title.trim() || content.trim()) {
      onSave(title.trim(), content.trim());
      setTitle('');
      setContent('');
      onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <h2>Quick Add Note</h2>
        
        <input
          ref={titleRef}
          type="text"
          placeholder="Title (optional)"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          className="modal-input title-input"
        />
        
        <textarea
          placeholder="Content... Use #tags for organization"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          className="modal-textarea"
          rows={6}
        />
        
        <div className="modal-actions">
          <button onClick={onClose} className="btn-cancel">Cancel</button>
          <button 
            onClick={handleSave} 
            disabled={!title.trim() && !content.trim()}
            className="btn-save"
          >
            Save (Ctrl+Enter)
          </button>
        </div>
      </div>
    </div>
  );
};
