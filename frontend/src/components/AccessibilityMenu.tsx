import React, { useState, useEffect } from 'react';
import { accessibilityManager } from '../utils/accessibility';
import { keyboardManager, KeyboardShortcut } from '../utils/keyboard';

interface AccessibilityMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AccessibilityMenu: React.FC<AccessibilityMenuProps> = ({ isOpen, onClose }) => {
  const [activeTab, setActiveTab] = useState<'general' | 'shortcuts'>('general');
  const [shortcuts, setShortcuts] = useState<KeyboardShortcut[]>([]);
  const [editingShortcut, setEditingShortcut] = useState<string | null>(null);
  const [recordingKeys, setRecordingKeys] = useState<string[]>([]);

  // Accessibility preferences state
  const [highContrast, setHighContrast] = useState(accessibilityManager.isHighContrastEnabled());
  const [fontSize, setFontSize] = useState(accessibilityManager.getFontSize());
  const [reducedMotion, setReducedMotion] = useState(accessibilityManager.isReducedMotionEnabled());
  const [screenReader, setScreenReader] = useState(accessibilityManager.isScreenReaderModeEnabled());

  useEffect(() => {
    if (isOpen) {
      setShortcuts(keyboardManager.getShortcuts());
      // Trap focus in modal
      const modal = document.getElementById('accessibility-menu');
      if (modal) {
        accessibilityManager.trapFocus(modal);
      }
    }
  }, [isOpen]);

  useEffect(() => {
    if (editingShortcut && recordingKeys.length > 0) {
      const handleKeyUp = () => {
        if (recordingKeys.length > 0) {
          keyboardManager.updateShortcut(editingShortcut, recordingKeys);
          setShortcuts(keyboardManager.getShortcuts());
          setEditingShortcut(null);
          setRecordingKeys([]);
        }
      };

      window.addEventListener('keyup', handleKeyUp);
      return () => window.removeEventListener('keyup', handleKeyUp);
    }
  }, [editingShortcut, recordingKeys]);

  const handleHighContrastToggle = () => {
    accessibilityManager.toggleHighContrast();
    setHighContrast(!highContrast);
  };

  const handleFontSizeChange = (newSize: number) => {
    if (newSize > fontSize) {
      accessibilityManager.increaseFontSize();
    } else if (newSize < fontSize) {
      accessibilityManager.decreaseFontSize();
    }
    setFontSize(accessibilityManager.getFontSize());
  };

  const handleReducedMotionToggle = () => {
    accessibilityManager.toggleReducedMotion();
    setReducedMotion(!reducedMotion);
  };

  const handleScreenReaderToggle = () => {
    accessibilityManager.toggleScreenReaderMode();
    setScreenReader(!screenReader);
  };

  const handleShortcutEdit = (shortcutId: string) => {
    setEditingShortcut(shortcutId);
    setRecordingKeys([]);
    accessibilityManager.announce('Press the new key combination for this shortcut');
  };

  const handleKeyRecord = (e: React.KeyboardEvent) => {
    if (!editingShortcut) return;
    
    e.preventDefault();
    e.stopPropagation();

    const keys: string[] = [];
    if (e.ctrlKey) keys.push('Ctrl');
    if (e.metaKey) keys.push('Meta');
    if (e.altKey) keys.push('Alt');
    if (e.shiftKey) keys.push('Shift');
    
    const key = e.key.length === 1 ? e.key.toUpperCase() : e.key;
    if (!['Control', 'Meta', 'Alt', 'Shift'].includes(e.key)) {
      keys.push(key);
    }

    setRecordingKeys(keys);
  };

  const resetShortcuts = () => {
    if (window.confirm('Reset all keyboard shortcuts to defaults?')) {
      localStorage.removeItem('keyboard-shortcuts');
      window.location.reload();
    }
  };

  const renderGeneralSettings = () => (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold mb-4">Visual Settings</h3>
      
      <div className="space-y-4">
        <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <div>
            <label htmlFor="high-contrast" className="font-medium">
              High Contrast Mode
            </label>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Increase contrast for better visibility
            </p>
          </div>
          <button
            id="high-contrast"
            role="switch"
            aria-checked={highContrast}
            onClick={handleHighContrastToggle}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              highContrast ? 'bg-blue-600' : 'bg-gray-300'
            }`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                highContrast ? 'translate-x-6' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div className="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <label htmlFor="font-size" className="font-medium block mb-2">
            Font Size: {fontSize}%
          </label>
          <div className="flex items-center space-x-4">
            <button
              onClick={() => handleFontSizeChange(fontSize - 10)}
              className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
              aria-label="Decrease font size"
            >
              A-
            </button>
            <input
              id="font-size"
              type="range"
              min="50"
              max="200"
              step="10"
              value={fontSize}
              onChange={(e) => handleFontSizeChange(Number(e.target.value))}
              className="flex-1"
              aria-label="Font size slider"
            />
            <button
              onClick={() => handleFontSizeChange(fontSize + 10)}
              className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
              aria-label="Increase font size"
            >
              A+
            </button>
          </div>
          <button
            onClick={() => {
              accessibilityManager.resetFontSize();
              setFontSize(100);
            }}
            className="mt-2 text-sm text-blue-600 hover:underline"
          >
            Reset to default
          </button>
        </div>

        <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <div>
            <label htmlFor="reduced-motion" className="font-medium">
              Reduce Motion
            </label>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Minimize animations and transitions
            </p>
          </div>
          <button
            id="reduced-motion"
            role="switch"
            aria-checked={reducedMotion}
            onClick={handleReducedMotionToggle}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              reducedMotion ? 'bg-blue-600' : 'bg-gray-300'
            }`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                reducedMotion ? 'translate-x-6' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
          <div>
            <label htmlFor="screen-reader" className="font-medium">
              Screen Reader Mode
            </label>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Enhanced descriptions for screen readers
            </p>
          </div>
          <button
            id="screen-reader"
            role="switch"
            aria-checked={screenReader}
            onClick={handleScreenReaderToggle}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              screenReader ? 'bg-blue-600' : 'bg-gray-300'
            }`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                screenReader ? 'translate-x-6' : 'translate-x-1'
              }`}
            />
          </button>
        </div>
      </div>
    </div>
  );

  const renderKeyboardShortcuts = () => {
    const categories: KeyboardShortcut['category'][] = ['navigation', 'playback', 'editing', 'view', 'accessibility'];
    
    return (
      <div className="space-y-6">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-semibold">Keyboard Shortcuts</h3>
          <button
            onClick={resetShortcuts}
            className="text-sm text-blue-600 hover:underline"
          >
            Reset to defaults
          </button>
        </div>

        {categories.map(category => {
          const categoryShortcuts = shortcuts.filter(s => s.category === category);
          if (categoryShortcuts.length === 0) return null;

          return (
            <div key={category} className="space-y-2">
              <h4 className="font-medium capitalize text-gray-700 dark:text-gray-300">
                {category}
              </h4>
              <div className="space-y-1">
                {categoryShortcuts.map(shortcut => (
                  <div
                    key={shortcut.id}
                    className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded"
                  >
                    <div className="flex-1">
                      <div className="font-medium">{shortcut.name}</div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        {shortcut.description}
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      {editingShortcut === shortcut.id ? (
                        <input
                          type="text"
                          value={recordingKeys.join(' + ')}
                          onKeyDown={handleKeyRecord}
                          placeholder="Press keys..."
                          className="px-3 py-1 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                          autoFocus
                          aria-label={`Recording new shortcut for ${shortcut.name}`}
                        />
                      ) : (
                        <>
                          <kbd className="px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded text-sm font-mono">
                            {shortcut.keys.join(' + ')}
                          </kbd>
                          <button
                            onClick={() => handleShortcutEdit(shortcut.id)}
                            className="text-sm text-blue-600 hover:underline"
                            aria-label={`Edit shortcut for ${shortcut.name}`}
                          >
                            Edit
                          </button>
                        </>
                      )}
                      <button
                        onClick={() => {
                          if (shortcut.enabled) {
                            keyboardManager.disableShortcut(shortcut.id);
                          } else {
                            keyboardManager.enableShortcut(shortcut.id);
                          }
                          setShortcuts(keyboardManager.getShortcuts());
                        }}
                        className={`text-sm ${
                          shortcut.enabled !== false ? 'text-red-600' : 'text-green-600'
                        } hover:underline`}
                        aria-label={`${shortcut.enabled !== false ? 'Disable' : 'Enable'} ${shortcut.name}`}
                      >
                        {shortcut.enabled !== false ? 'Disable' : 'Enable'}
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>
    );
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={onClose}
      role="dialog"
      aria-labelledby="accessibility-menu-title"
      aria-modal="true"
    >
      <div
        id="accessibility-menu"
        className="bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-3xl w-full max-h-[80vh] overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 id="accessibility-menu-title" className="text-2xl font-bold">
            Accessibility Settings
          </h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
            aria-label="Close accessibility settings"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="flex border-b border-gray-200 dark:border-gray-700">
          <button
            onClick={() => setActiveTab('general')}
            className={`flex-1 px-6 py-3 text-center font-medium transition-colors ${
              activeTab === 'general'
                ? 'bg-gray-100 dark:bg-gray-800 text-blue-600 border-b-2 border-blue-600'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            }`}
            role="tab"
            aria-selected={activeTab === 'general'}
            aria-controls="general-panel"
          >
            General
          </button>
          <button
            onClick={() => setActiveTab('shortcuts')}
            className={`flex-1 px-6 py-3 text-center font-medium transition-colors ${
              activeTab === 'shortcuts'
                ? 'bg-gray-100 dark:bg-gray-800 text-blue-600 border-b-2 border-blue-600'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
            }`}
            role="tab"
            aria-selected={activeTab === 'shortcuts'}
            aria-controls="shortcuts-panel"
          >
            Keyboard Shortcuts
          </button>
        </div>

        <div className="p-6 overflow-y-auto max-h-[calc(80vh-200px)]">
          {activeTab === 'general' ? (
            <div id="general-panel" role="tabpanel" tabIndex={0}>
              {renderGeneralSettings()}
            </div>
          ) : (
            <div id="shortcuts-panel" role="tabpanel" tabIndex={0}>
              {renderKeyboardShortcuts()}
            </div>
          )}
        </div>

        <div className="flex justify-end p-6 border-t border-gray-200 dark:border-gray-700">
          <button
            onClick={onClose}
            className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};