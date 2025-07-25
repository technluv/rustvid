import { useEffect, useCallback, useState } from 'react';
import { keyboardManager, KeyboardShortcut, defaultShortcuts } from '../utils/keyboard';
import { accessibilityManager } from '../utils/accessibility';

interface UseKeyboardShortcutsOptions {
  enabled?: boolean;
  customShortcuts?: Partial<KeyboardShortcut>[];
}

export function useKeyboardShortcuts(
  actions: Record<string, () => void>,
  options: UseKeyboardShortcutsOptions = {}
) {
  const { enabled = true, customShortcuts = [] } = options;
  const [isShortcutsDialogOpen, setShortcutsDialogOpen] = useState(false);

  // Register default shortcuts with actions
  useEffect(() => {
    if (!enabled) return;

    // Navigation shortcuts
    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'nav-timeline')!,
      action: () => {
        const timeline = document.querySelector('[data-component="timeline"]') as HTMLElement;
        if (timeline) {
          accessibilityManager.setFocus(timeline);
        }
      }
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'nav-preview')!,
      action: () => {
        const preview = document.querySelector('[data-component="preview"]') as HTMLElement;
        if (preview) {
          accessibilityManager.setFocus(preview);
        }
      }
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'nav-properties')!,
      action: () => {
        const properties = document.querySelector('[data-component="properties"]') as HTMLElement;
        if (properties) {
          accessibilityManager.setFocus(properties);
        }
      }
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'nav-media')!,
      action: () => {
        const media = document.querySelector('[data-component="media-library"]') as HTMLElement;
        if (media) {
          accessibilityManager.setFocus(media);
        }
      }
    });

    // Playback shortcuts
    if (actions.playPause) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'play-pause')!,
        action: actions.playPause
      });
    }

    if (actions.stop) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'stop')!,
        action: actions.stop
      });
    }

    if (actions.nextFrame) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'next-frame')!,
        action: actions.nextFrame
      });
    }

    if (actions.prevFrame) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'prev-frame')!,
        action: actions.prevFrame
      });
    }

    // Editing shortcuts
    if (actions.cut) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'cut')!,
        action: actions.cut
      });
    }

    if (actions.copy) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'copy')!,
        action: actions.copy
      });
    }

    if (actions.paste) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'paste')!,
        action: actions.paste
      });
    }

    if (actions.delete) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'delete')!,
        action: actions.delete
      });
    }

    if (actions.undo) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'undo')!,
        action: actions.undo
      });
    }

    if (actions.redo) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'redo')!,
        action: actions.redo
      });
    }

    // View shortcuts
    if (actions.zoomIn) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'zoom-in')!,
        action: actions.zoomIn
      });
    }

    if (actions.zoomOut) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'zoom-out')!,
        action: actions.zoomOut
      });
    }

    if (actions.fitToWindow) {
      keyboardManager.registerShortcut({
        ...defaultShortcuts.find(s => s.id === 'fit-to-window')!,
        action: actions.fitToWindow
      });
    }

    // Accessibility shortcuts
    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'toggle-high-contrast')!,
      action: () => accessibilityManager.toggleHighContrast()
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'increase-font')!,
      action: () => accessibilityManager.increaseFontSize()
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'decrease-font')!,
      action: () => accessibilityManager.decreaseFontSize()
    });

    keyboardManager.registerShortcut({
      ...defaultShortcuts.find(s => s.id === 'show-shortcuts')!,
      action: () => setShortcutsDialogOpen(true)
    });

    // Register custom shortcuts
    customShortcuts.forEach(shortcut => {
      if (shortcut.id && shortcut.action) {
        keyboardManager.registerShortcut(shortcut as KeyboardShortcut);
      }
    });

    // Cleanup
    return () => {
      defaultShortcuts.forEach(s => keyboardManager.unregisterShortcut(s.id));
      customShortcuts.forEach(s => s.id && keyboardManager.unregisterShortcut(s.id));
    };
  }, [enabled, actions, customShortcuts]);

  // Listen for shortcut triggers
  useEffect(() => {
    const unsubscribe = keyboardManager.onShortcutTriggered((shortcut) => {
      accessibilityManager.announceAction(shortcut.name);
    });

    return unsubscribe;
  }, []);

  const getShortcuts = useCallback(() => {
    return keyboardManager.getShortcuts();
  }, []);

  const getShortcutsByCategory = useCallback((category: KeyboardShortcut['category']) => {
    return keyboardManager.getShortcutsByCategory(category);
  }, []);

  const updateShortcut = useCallback((id: string, keys: string[]) => {
    keyboardManager.updateShortcut(id, keys);
  }, []);

  const enableShortcut = useCallback((id: string) => {
    keyboardManager.enableShortcut(id);
  }, []);

  const disableShortcut = useCallback((id: string) => {
    keyboardManager.disableShortcut(id);
  }, []);

  return {
    shortcuts: getShortcuts(),
    getShortcutsByCategory,
    updateShortcut,
    enableShortcut,
    disableShortcut,
    isShortcutsDialogOpen,
    setShortcutsDialogOpen
  };
}

// Specific hooks for common patterns
export function usePlaybackShortcuts(
  playPause: () => void,
  stop: () => void,
  nextFrame: () => void,
  prevFrame: () => void
) {
  return useKeyboardShortcuts({
    playPause,
    stop,
    nextFrame,
    prevFrame
  });
}

export function useEditingShortcuts(
  cut: () => void,
  copy: () => void,
  paste: () => void,
  deleteClip: () => void,
  undo: () => void,
  redo: () => void
) {
  return useKeyboardShortcuts({
    cut,
    copy,
    paste,
    delete: deleteClip,
    undo,
    redo
  });
}

export function useViewShortcuts(
  zoomIn: () => void,
  zoomOut: () => void,
  fitToWindow: () => void
) {
  return useKeyboardShortcuts({
    zoomIn,
    zoomOut,
    fitToWindow
  });
}

// Hook for global app shortcuts
export function useGlobalShortcuts() {
  const [shortcuts, setShortcuts] = useState<KeyboardShortcut[]>([]);

  useEffect(() => {
    // Load shortcuts from storage or use defaults
    const savedShortcuts = localStorage.getItem('keyboard-shortcuts');
    if (savedShortcuts) {
      try {
        const parsed = JSON.parse(savedShortcuts);
        setShortcuts(parsed);
      } catch {
        setShortcuts(keyboardManager.getShortcuts());
      }
    } else {
      setShortcuts(keyboardManager.getShortcuts());
    }
  }, []);

  const saveShortcuts = useCallback((newShortcuts: KeyboardShortcut[]) => {
    localStorage.setItem('keyboard-shortcuts', JSON.stringify(newShortcuts));
    setShortcuts(newShortcuts);
  }, []);

  const resetShortcuts = useCallback(() => {
    localStorage.removeItem('keyboard-shortcuts');
    window.location.reload(); // Reload to apply defaults
  }, []);

  return {
    shortcuts,
    saveShortcuts,
    resetShortcuts
  };
}