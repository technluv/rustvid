// Keyboard shortcut manager for comprehensive keyboard navigation
export interface KeyboardShortcut {
  id: string;
  name: string;
  description: string;
  keys: string[];
  action: () => void;
  category: 'navigation' | 'playback' | 'editing' | 'view' | 'accessibility';
  enabled?: boolean;
}

export class KeyboardShortcutManager {
  private shortcuts: Map<string, KeyboardShortcut> = new Map();
  private activeKeys: Set<string> = new Set();
  private listeners: Map<string, (shortcut: KeyboardShortcut) => void> = new Map();

  constructor() {
    this.setupEventListeners();
  }

  private setupEventListeners() {
    window.addEventListener('keydown', this.handleKeyDown.bind(this));
    window.addEventListener('keyup', this.handleKeyUp.bind(this));
    window.addEventListener('blur', this.clearActiveKeys.bind(this));
  }

  private handleKeyDown(event: KeyboardEvent) {
    // Prevent shortcuts in input fields unless explicitly allowed
    if (this.shouldIgnoreEvent(event)) return;

    const key = this.normalizeKey(event);
    this.activeKeys.add(key);

    const shortcut = this.findMatchingShortcut();
    if (shortcut && shortcut.enabled !== false) {
      event.preventDefault();
      event.stopPropagation();
      shortcut.action();
      this.notifyListeners(shortcut);
    }
  }

  private handleKeyUp(event: KeyboardEvent) {
    const key = this.normalizeKey(event);
    this.activeKeys.delete(key);
  }

  private clearActiveKeys() {
    this.activeKeys.clear();
  }

  private shouldIgnoreEvent(event: KeyboardEvent): boolean {
    const target = event.target as HTMLElement;
    const isInput = ['INPUT', 'TEXTAREA', 'SELECT'].includes(target.tagName);
    const isContentEditable = target.contentEditable === 'true';
    
    // Allow some shortcuts in input fields (e.g., Escape)
    if (event.key === 'Escape') return false;
    
    return isInput || isContentEditable;
  }

  private normalizeKey(event: KeyboardEvent): string {
    const parts: string[] = [];
    
    if (event.ctrlKey) parts.push('Ctrl');
    if (event.metaKey) parts.push('Meta');
    if (event.altKey) parts.push('Alt');
    if (event.shiftKey) parts.push('Shift');
    
    const key = event.key.length === 1 ? event.key.toUpperCase() : event.key;
    parts.push(key);
    
    return parts.join('+');
  }

  private findMatchingShortcut(): KeyboardShortcut | undefined {
    const activeKeyArray = Array.from(this.activeKeys);
    
    for (const shortcut of this.shortcuts.values()) {
      if (this.keysMatch(shortcut.keys, activeKeyArray)) {
        return shortcut;
      }
    }
    
    return undefined;
  }

  private keysMatch(shortcutKeys: string[], activeKeys: string[]): boolean {
    if (shortcutKeys.length !== activeKeys.length) return false;
    
    const shortcutSet = new Set(shortcutKeys);
    const activeSet = new Set(activeKeys);
    
    for (const key of shortcutSet) {
      if (!activeSet.has(key)) return false;
    }
    
    return true;
  }

  private notifyListeners(shortcut: KeyboardShortcut) {
    this.listeners.forEach(listener => listener(shortcut));
  }

  registerShortcut(shortcut: KeyboardShortcut) {
    this.shortcuts.set(shortcut.id, shortcut);
  }

  unregisterShortcut(id: string) {
    this.shortcuts.delete(id);
  }

  updateShortcut(id: string, keys: string[]) {
    const shortcut = this.shortcuts.get(id);
    if (shortcut) {
      shortcut.keys = keys;
    }
  }

  enableShortcut(id: string) {
    const shortcut = this.shortcuts.get(id);
    if (shortcut) {
      shortcut.enabled = true;
    }
  }

  disableShortcut(id: string) {
    const shortcut = this.shortcuts.get(id);
    if (shortcut) {
      shortcut.enabled = false;
    }
  }

  getShortcuts(): KeyboardShortcut[] {
    return Array.from(this.shortcuts.values());
  }

  getShortcutsByCategory(category: KeyboardShortcut['category']): KeyboardShortcut[] {
    return this.getShortcuts().filter(s => s.category === category);
  }

  onShortcutTriggered(listener: (shortcut: KeyboardShortcut) => void): () => void {
    const id = Math.random().toString(36);
    this.listeners.set(id, listener);
    
    return () => {
      this.listeners.delete(id);
    };
  }

  destroy() {
    window.removeEventListener('keydown', this.handleKeyDown.bind(this));
    window.removeEventListener('keyup', this.handleKeyUp.bind(this));
    window.removeEventListener('blur', this.clearActiveKeys.bind(this));
    this.shortcuts.clear();
    this.activeKeys.clear();
    this.listeners.clear();
  }
}

// Singleton instance
export const keyboardManager = new KeyboardShortcutManager();

// Default keyboard shortcuts
export const defaultShortcuts: Omit<KeyboardShortcut, 'action'>[] = [
  // Navigation
  {
    id: 'nav-timeline',
    name: 'Focus Timeline',
    description: 'Move focus to timeline',
    keys: ['Alt', '1'],
    category: 'navigation'
  },
  {
    id: 'nav-preview',
    name: 'Focus Preview',
    description: 'Move focus to preview window',
    keys: ['Alt', '2'],
    category: 'navigation'
  },
  {
    id: 'nav-properties',
    name: 'Focus Properties',
    description: 'Move focus to properties panel',
    keys: ['Alt', '3'],
    category: 'navigation'
  },
  {
    id: 'nav-media',
    name: 'Focus Media Library',
    description: 'Move focus to media library',
    keys: ['Alt', '4'],
    category: 'navigation'
  },
  // Playback
  {
    id: 'play-pause',
    name: 'Play/Pause',
    description: 'Toggle playback',
    keys: ['Space'],
    category: 'playback'
  },
  {
    id: 'stop',
    name: 'Stop',
    description: 'Stop playback',
    keys: ['Escape'],
    category: 'playback'
  },
  {
    id: 'next-frame',
    name: 'Next Frame',
    description: 'Move to next frame',
    keys: ['ArrowRight'],
    category: 'playback'
  },
  {
    id: 'prev-frame',
    name: 'Previous Frame',
    description: 'Move to previous frame',
    keys: ['ArrowLeft'],
    category: 'playback'
  },
  // Editing
  {
    id: 'cut',
    name: 'Cut',
    description: 'Cut selected clip',
    keys: ['Ctrl', 'X'],
    category: 'editing'
  },
  {
    id: 'copy',
    name: 'Copy',
    description: 'Copy selected clip',
    keys: ['Ctrl', 'C'],
    category: 'editing'
  },
  {
    id: 'paste',
    name: 'Paste',
    description: 'Paste clip',
    keys: ['Ctrl', 'V'],
    category: 'editing'
  },
  {
    id: 'delete',
    name: 'Delete',
    description: 'Delete selected clip',
    keys: ['Delete'],
    category: 'editing'
  },
  {
    id: 'undo',
    name: 'Undo',
    description: 'Undo last action',
    keys: ['Ctrl', 'Z'],
    category: 'editing'
  },
  {
    id: 'redo',
    name: 'Redo',
    description: 'Redo last undone action',
    keys: ['Ctrl', 'Shift', 'Z'],
    category: 'editing'
  },
  // View
  {
    id: 'zoom-in',
    name: 'Zoom In',
    description: 'Zoom in timeline',
    keys: ['Ctrl', '='],
    category: 'view'
  },
  {
    id: 'zoom-out',
    name: 'Zoom Out',
    description: 'Zoom out timeline',
    keys: ['Ctrl', '-'],
    category: 'view'
  },
  {
    id: 'fit-to-window',
    name: 'Fit to Window',
    description: 'Fit timeline to window',
    keys: ['Ctrl', '0'],
    category: 'view'
  },
  // Accessibility
  {
    id: 'toggle-high-contrast',
    name: 'Toggle High Contrast',
    description: 'Toggle high contrast mode',
    keys: ['Alt', 'H'],
    category: 'accessibility'
  },
  {
    id: 'increase-font',
    name: 'Increase Font Size',
    description: 'Increase UI font size',
    keys: ['Ctrl', 'Shift', '='],
    category: 'accessibility'
  },
  {
    id: 'decrease-font',
    name: 'Decrease Font Size',
    description: 'Decrease UI font size',
    keys: ['Ctrl', 'Shift', '-'],
    category: 'accessibility'
  },
  {
    id: 'show-shortcuts',
    name: 'Show Keyboard Shortcuts',
    description: 'Display keyboard shortcuts dialog',
    keys: ['?'],
    category: 'accessibility'
  }
];