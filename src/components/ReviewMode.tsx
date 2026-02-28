import React, { useState, useEffect } from 'react';

interface Card {
  id: number;
  title: string;
  content: string;
  knowledge_type: string;
}

interface ReviewModeProps {
  isOpen: boolean;
  onClose: () => void;
  onCardComplete: (cardId: number, rating: 'again' | 'hard' | 'good' | 'easy') => void;
}

export const ReviewMode: React.FC<ReviewModeProps> = ({ 
  isOpen, 
  onClose, 
  onCardComplete 
}) => {
  const [cards, setCards] = useState<Card[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isFlipped, setIsFlipped] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadReviewCards();
    }
  }, [isOpen]);

  const loadReviewCards = async () => {
    try {
      // In production: fetch cards due for review from backend
      const result = await window.__TAURI__.invoke('get_review_cards', {});
      setCards(result || []);
      setCurrentIndex(0);
      setIsFlipped(false);
    } catch (error) {
      console.error('Failed to load review cards:', error);
      alert('Failed to load review cards. Please try again.');
    }
  };

  const currentCard = cards[currentIndex];

  const handleNext = () => {
    setIsFlipped(false);
    
    if (currentIndex < cards.length - 1) {
      setCurrentIndex(currentIndex + 1);
    } else {
      // All cards completed
      onClose();
    }
  };

  const handleRating = (rating: 'again' | 'hard' | 'good' | 'easy') => {
    if (currentCard) {
      onCardComplete(currentCard.id, rating);
    }
    handleNext();
  };

  const handleClose = () => {
    setIsFlipped(false);
    onClose();
  };

  if (!isOpen || cards.length === 0) return null;

  const progress = ((currentIndex + (isFlipped ? 1 : 0)) / cards.length) * 100;

  return (
    <div className="modal-overlay review-modal" onClick={handleClose}>
      <div className="review-content" onClick={(e) => e.stopPropagation()}>
        {/* Progress Bar */}
        <div className="progress-bar">
          <div 
            className="progress-fill" 
            style={{ width: `${progress}%` }}
          />
        </div>

        {/* Card Count */}
        <div className="card-counter">
          {currentIndex + 1} / {cards.length} cards remaining
        </div>

        {/* Main Card Area */}
        <div className={`flashcard ${isFlipped ? 'flipped' : ''}`}>
          <div className="card-face front">
            <h2>{currentCard?.title}</h2>
            <span className="card-type">{currentCard?.knowledge_type}</span>
            
            <button 
              onClick={() => setIsFlipped(true)}
              className="flip-btn"
            >
              Click to reveal answer
            </button>
          </div>

          <div className="card-face back">
            <h2>Answer</h2>
            <pre>{currentCard?.content}</pre>
            
            <button 
              onClick={() => setIsFlipped(false)}
              className="flip-back-btn"
            >
              Flip back
            </button>
          </div>
        </div>

        {/* Rating Buttons (shown after flip) */}
        {isFlipped && (
          <div className="rating-buttons">
            <button 
              onClick={() => handleRating('again')}
              className="rating-btn again"
              title="Review again soon"
            >
              😞 Again
            </button>
            
            <button 
              onClick={() => handleRating('hard')}
              className="rating-btn hard"
              title="Show later"
            >
              🤔 Hard
            </button>
            
            <button 
              onClick={() => handleRating('good')}
              className="rating-btn good"
              title="Show in a few days"
            >
              🙂 Good
            </button>
            
            <button 
              onClick={() => handleRating('easy')}
              className="rating-btn easy"
              title="Show in several weeks"
            >
              😄 Easy
            </button>
          </div>
        )}

        {/* Close Button */}
        {!isFlipped && (
          <button onClick={handleClose} className="review-close">
            Close Review Mode
          </button>
        )}
      </div>
    </div>
  );
};
