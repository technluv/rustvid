// Accessibility utilities for screen readers, high contrast, and more
export class AccessibilityManager {
  private announcer: HTMLDivElement;
  private highContrastEnabled: boolean = false;
  private fontSize: number = 100; // percentage
  private keyboardNavEnabled: boolean = true;
  private reducedMotion: boolean = false;
  private screenReaderMode: boolean = false;

  constructor() {
    this.announcer = this.createAnnouncer();
    this.loadPreferences();
    this.detectSystemPreferences();
  }

  private createAnnouncer(): HTMLDivElement {
    const announcer = document.createElement('div');
    announcer.setAttribute('role', 'status');
    announcer.setAttribute('aria-live', 'polite');
    announcer.setAttribute('aria-atomic', 'true');
    announcer.className = 'sr-only';
    announcer.style.position = 'absolute';
    announcer.style.left = '-10000px';
    announcer.style.width = '1px';
    announcer.style.height = '1px';
    announcer.style.overflow = 'hidden';
    document.body.appendChild(announcer);
    return announcer;
  }

  private loadPreferences() {
    const prefs = localStorage.getItem('accessibility-preferences');
    if (prefs) {
      const parsed = JSON.parse(prefs);
      this.highContrastEnabled = parsed.highContrast ?? false;
      this.fontSize = parsed.fontSize ?? 100;
      this.keyboardNavEnabled = parsed.keyboardNav ?? true;
      this.reducedMotion = parsed.reducedMotion ?? false;
      this.screenReaderMode = parsed.screenReader ?? false;
      this.applyPreferences();
    }
  }

  private savePreferences() {
    const prefs = {
      highContrast: this.highContrastEnabled,
      fontSize: this.fontSize,
      keyboardNav: this.keyboardNavEnabled,
      reducedMotion: this.reducedMotion,
      screenReader: this.screenReaderMode
    };
    localStorage.setItem('accessibility-preferences', JSON.stringify(prefs));
  }

  private detectSystemPreferences() {
    // Detect high contrast
    if (window.matchMedia('(prefers-contrast: high)').matches) {
      this.enableHighContrast();
    }

    // Detect reduced motion
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
      this.enableReducedMotion();
    }

    // Listen for changes
    window.matchMedia('(prefers-contrast: high)').addEventListener('change', (e) => {
      if (e.matches) this.enableHighContrast();
      else this.disableHighContrast();
    });

    window.matchMedia('(prefers-reduced-motion: reduce)').addEventListener('change', (e) => {
      if (e.matches) this.enableReducedMotion();
      else this.disableReducedMotion();
    });
  }

  private applyPreferences() {
    // Apply high contrast
    if (this.highContrastEnabled) {
      document.documentElement.classList.add('high-contrast');
    } else {
      document.documentElement.classList.remove('high-contrast');
    }

    // Apply font size
    document.documentElement.style.setProperty('--base-font-size', `${this.fontSize}%`);

    // Apply reduced motion
    if (this.reducedMotion) {
      document.documentElement.classList.add('reduced-motion');
    } else {
      document.documentElement.classList.remove('reduced-motion');
    }

    // Apply screen reader mode
    if (this.screenReaderMode) {
      document.documentElement.classList.add('screen-reader-mode');
    } else {
      document.documentElement.classList.remove('screen-reader-mode');
    }
  }

  // Screen reader announcements
  announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
    this.announcer.setAttribute('aria-live', priority);
    this.announcer.textContent = message;
    
    // Clear after announcement
    setTimeout(() => {
      this.announcer.textContent = '';
    }, 1000);
  }

  announceAction(action: string, target?: string) {
    const message = target ? `${action} ${target}` : action;
    this.announce(message);
  }

  // High contrast mode
  enableHighContrast() {
    this.highContrastEnabled = true;
    document.documentElement.classList.add('high-contrast');
    this.savePreferences();
    this.announce('High contrast mode enabled');
  }

  disableHighContrast() {
    this.highContrastEnabled = false;
    document.documentElement.classList.remove('high-contrast');
    this.savePreferences();
    this.announce('High contrast mode disabled');
  }

  toggleHighContrast() {
    if (this.highContrastEnabled) {
      this.disableHighContrast();
    } else {
      this.enableHighContrast();
    }
  }

  // Font size management
  increaseFontSize() {
    this.fontSize = Math.min(this.fontSize + 10, 200);
    this.applyPreferences();
    this.savePreferences();
    this.announce(`Font size increased to ${this.fontSize}%`);
  }

  decreaseFontSize() {
    this.fontSize = Math.max(this.fontSize - 10, 50);
    this.applyPreferences();
    this.savePreferences();
    this.announce(`Font size decreased to ${this.fontSize}%`);
  }

  resetFontSize() {
    this.fontSize = 100;
    this.applyPreferences();
    this.savePreferences();
    this.announce('Font size reset to default');
  }

  // Reduced motion
  enableReducedMotion() {
    this.reducedMotion = true;
    document.documentElement.classList.add('reduced-motion');
    this.savePreferences();
    this.announce('Reduced motion enabled');
  }

  disableReducedMotion() {
    this.reducedMotion = false;
    document.documentElement.classList.remove('reduced-motion');
    this.savePreferences();
    this.announce('Reduced motion disabled');
  }

  toggleReducedMotion() {
    if (this.reducedMotion) {
      this.disableReducedMotion();
    } else {
      this.enableReducedMotion();
    }
  }

  // Screen reader mode
  enableScreenReaderMode() {
    this.screenReaderMode = true;
    document.documentElement.classList.add('screen-reader-mode');
    this.savePreferences();
    this.announce('Screen reader mode enabled. Additional descriptions will be provided.');
  }

  disableScreenReaderMode() {
    this.screenReaderMode = false;
    document.documentElement.classList.remove('screen-reader-mode');
    this.savePreferences();
    this.announce('Screen reader mode disabled');
  }

  toggleScreenReaderMode() {
    if (this.screenReaderMode) {
      this.disableScreenReaderMode();
    } else {
      this.enableScreenReaderMode();
    }
  }

  // Focus management
  setFocus(element: HTMLElement | null, announce: boolean = true) {
    if (!element) return;

    element.focus();
    
    if (announce) {
      const label = element.getAttribute('aria-label') || 
                   element.textContent || 
                   element.getAttribute('title') || 
                   'element';
      this.announce(`Focused on ${label}`);
    }
  }

  trapFocus(container: HTMLElement) {
    const focusableElements = container.querySelectorAll(
      'a[href], button, textarea, input[type="text"], input[type="radio"], input[type="checkbox"], select, [tabindex]:not([tabindex="-1"])'
    );
    
    const firstFocusable = focusableElements[0] as HTMLElement;
    const lastFocusable = focusableElements[focusableElements.length - 1] as HTMLElement;

    container.addEventListener('keydown', (e) => {
      if (e.key !== 'Tab') return;

      if (e.shiftKey) {
        if (document.activeElement === firstFocusable) {
          e.preventDefault();
          lastFocusable.focus();
        }
      } else {
        if (document.activeElement === lastFocusable) {
          e.preventDefault();
          firstFocusable.focus();
        }
      }
    });
  }

  // Skip navigation
  createSkipLink(targetId: string, text: string = 'Skip to main content'): HTMLAnchorElement {
    const link = document.createElement('a');
    link.href = `#${targetId}`;
    link.className = 'skip-link';
    link.textContent = text;
    link.addEventListener('click', (e) => {
      e.preventDefault();
      const target = document.getElementById(targetId);
      if (target) {
        this.setFocus(target);
      }
    });
    return link;
  }

  // Getters
  isHighContrastEnabled(): boolean {
    return this.highContrastEnabled;
  }

  getFontSize(): number {
    return this.fontSize;
  }

  isReducedMotionEnabled(): boolean {
    return this.reducedMotion;
  }

  isScreenReaderModeEnabled(): boolean {
    return this.screenReaderMode;
  }

  destroy() {
    if (this.announcer.parentNode) {
      this.announcer.parentNode.removeChild(this.announcer);
    }
  }
}

// Singleton instance
export const accessibilityManager = new AccessibilityManager();

// Utility functions
export function describeLiveRegion(element: HTMLElement, description: string) {
  element.setAttribute('aria-live', 'polite');
  element.setAttribute('aria-label', description);
}

export function makeAccessibleButton(element: HTMLElement, label: string, action?: string) {
  element.setAttribute('role', 'button');
  element.setAttribute('tabindex', '0');
  element.setAttribute('aria-label', label);
  
  if (action) {
    element.setAttribute('aria-description', action);
  }

  // Add keyboard support
  element.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      element.click();
    }
  });
}

export function addAriaLabels(container: HTMLElement, labels: Record<string, string>) {
  Object.entries(labels).forEach(([selector, label]) => {
    const element = container.querySelector(selector);
    if (element) {
      element.setAttribute('aria-label', label);
    }
  });
}