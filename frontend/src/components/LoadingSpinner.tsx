import React from 'react';
import { LoaderIcon } from 'lucide-react';

interface LoadingSpinnerProps {
  message?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  showProgress?: boolean;
  progress?: number;
}

export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  message = 'Loading...',
  size = 'md',
  className = '',
  showProgress = false,
  progress = 0,
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  const containerSizeClasses = {
    sm: 'text-sm',
    md: 'text-base',
    lg: 'text-lg',
  };

  return (
    <div 
      className={`flex items-center justify-center min-h-screen bg-gray-900 text-white ${className}`}
      role="status"
      aria-live="polite"
      aria-label={message}
    >
      <div className={`text-center ${containerSizeClasses[size]}`}>
        {/* Animated spinner */}
        <div className="flex justify-center mb-4">
          <LoaderIcon 
            className={`${sizeClasses[size]} animate-spin text-blue-500`} 
            aria-hidden="true"
          />
        </div>
        
        {/* Loading message */}
        <div className="font-medium mb-2">
          {message}
        </div>
        
        {/* Progress bar (optional) */}
        {showProgress && (
          <div className="w-64 bg-gray-700 rounded-full h-2 mb-4 overflow-hidden">
            <div 
              className="bg-blue-500 h-full rounded-full transition-all duration-300 ease-out"
              style={{ width: `${Math.max(0, Math.min(100, progress))}%` }}
              role="progressbar"
              aria-valuenow={progress}
              aria-valuemin={0}
              aria-valuemax={100}
              aria-label={`Loading progress: ${progress}%`}
            />
          </div>
        )}
        
        {/* Loading dots animation */}
        <div className="flex justify-center space-x-1" aria-hidden="true">
          <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
          <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
          <div className="w-2 h-2 bg-gray-500 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
        </div>
        
        {/* Accessible loading announcement */}
        <div className="sr-only" aria-live="polite">
          {message} Please wait...
        </div>
      </div>
    </div>
  );
};

// Inline loading spinner for smaller contexts
export const InlineSpinner: React.FC<Omit<LoadingSpinnerProps, 'className'> & { 
  className?: string;
  inline?: boolean; 
}> = ({
  message,
  size = 'sm',
  className = '',
  inline = true,
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-6 h-6',
    lg: 'w-8 h-8',
  };

  if (inline) {
    return (
      <span 
        className={`inline-flex items-center gap-2 ${className}`}
        role="status"
        aria-label={message || 'Loading'}
      >
        <LoaderIcon 
          className={`${sizeClasses[size]} animate-spin text-blue-500`} 
          aria-hidden="true"
        />
        {message && <span className="text-sm">{message}</span>}
      </span>
    );
  }

  return (
    <div 
      className={`flex items-center justify-center p-4 ${className}`}
      role="status"
      aria-label={message || 'Loading'}
    >
      <LoaderIcon 
        className={`${sizeClasses[size]} animate-spin text-blue-500 mr-2`} 
        aria-hidden="true"
      />
      {message && <span className="text-sm">{message}</span>}
    </div>
  );
};