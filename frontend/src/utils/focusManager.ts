// Focus management utilities for keyboard navigation
export class FocusManager {
  private focusableElements: HTMLElement[] = [];
  private currentIndex: number = -1;
  private container: HTMLElement | null = null;
  private focusTrapStack: HTMLElement[] = [];

  constructor() {
    this.handleKeyDown = this.handleKeyDown.bind(this);
  }

  // Initialize focus management for a container
  init(container: HTMLElement) {
    this.container = container;
    this.updateFocusableElements();
    container.addEventListener('keydown', this.handleKeyDown);
  }

  // Clean up event listeners
  destroy() {
    if (this.container) {
      this.container.removeEventListener('keydown', this.handleKeyDown);
    }
    this.focusableElements = [];
    this.currentIndex = -1;
    this.container = null;
  }

  // Update the list of focusable elements
  updateFocusableElements() {
    if (!this.container) return;

    const selector = [
      'a[href]',
      'button:not([disabled])',
      'textarea:not([disabled])',
      'input[type="text"]:not([disabled])',
      'input[type="radio"]:not([disabled])',
      'input[type="checkbox"]:not([disabled])',
      'select:not([disabled])',
      '[tabindex]:not([tabindex="-1"])',
      '[contenteditable="true"]'
    ].join(', ');

    const elements = this.container.querySelectorAll<HTMLElement>(selector);
    this.focusableElements = Array.from(elements).filter(el => {
      // Filter out elements that are not visible
      const rect = el.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0;
    });
  }

  // Handle keyboard navigation
  private handleKeyDown(event: KeyboardEvent) {
    // Skip if we're in a text input
    const target = event.target as HTMLElement;
    if (this.isTextInput(target) && !['Escape', 'Tab'].includes(event.key)) {
      return;
    }

    switch (event.key) {
      case 'Tab':
        this.handleTab(event);
        break;
      case 'Escape':
        this.handleEscape(event);
        break;
      case 'ArrowUp':
      case 'ArrowDown':
        if (this.isInGrid()) {
          this.handleArrowNavigation(event);
        }
        break;
      case 'Home':
        if (event.ctrlKey) {
          this.focusFirst();
          event.preventDefault();
        }
        break;
      case 'End':
        if (event.ctrlKey) {
          this.focusLast();
          event.preventDefault();
        }
        break;
    }
  }

  // Handle Tab key navigation
  private handleTab(event: KeyboardEvent) {
    if (this.focusTrapStack.length > 0) {
      // We're in a focus trap
      const trap = this.focusTrapStack[this.focusTrapStack.length - 1];
      this.handleTabInTrap(event, trap);
      return;
    }

    // Normal tab navigation
    this.updateFocusableElements();
    
    if (this.focusableElements.length === 0) return;

    const activeElement = document.activeElement as HTMLElement;
    const currentIndex = this.focusableElements.indexOf(activeElement);

    if (event.shiftKey) {
      // Shift+Tab - move backwards
      if (currentIndex <= 0) {
        this.focusableElements[this.focusableElements.length - 1].focus();
        event.preventDefault();
      }
    } else {
      // Tab - move forwards
      if (currentIndex === this.focusableElements.length - 1) {
        this.focusableElements[0].focus();
        event.preventDefault();
      }
    }
  }

  // Handle Tab in focus trap
  private handleTabInTrap(event: KeyboardEvent, trap: HTMLElement) {
    const selector = [
      'a[href]',
      'button:not([disabled])',
      'textarea:not([disabled])',
      'input:not([disabled])',
      'select:not([disabled])',
      '[tabindex]:not([tabindex="-1"])'
    ].join(', ');

    const focusable = Array.from(trap.querySelectorAll<HTMLElement>(selector));
    
    if (focusable.length === 0) return;

    const firstFocusable = focusable[0];
    const lastFocusable = focusable[focusable.length - 1];

    if (event.shiftKey && document.activeElement === firstFocusable) {
      lastFocusable.focus();
      event.preventDefault();
    } else if (!event.shiftKey && document.activeElement === lastFocusable) {
      firstFocusable.focus();
      event.preventDefault();
    }
  }

  // Handle Escape key
  private handleEscape(event: KeyboardEvent) {
    if (this.focusTrapStack.length > 0) {
      this.releaseFocusTrap();
      event.preventDefault();
    }
  }

  // Handle arrow key navigation in grids
  private handleArrowNavigation(event: KeyboardEvent) {
    const activeElement = document.activeElement as HTMLElement;
    const grid = activeElement.closest('[role="grid"]');
    
    if (!grid) return;

    const cells = Array.from(grid.querySelectorAll('[role="gridcell"], [role="cell"]')) as HTMLElement[];
    const currentIndex = cells.indexOf(activeElement);
    
    if (currentIndex === -1) return;

    const columns = parseInt(grid.getAttribute('aria-colcount') || '1');
    let newIndex = currentIndex;

    switch (event.key) {
      case 'ArrowUp':
        newIndex = currentIndex - columns;
        break;
      case 'ArrowDown':
        newIndex = currentIndex + columns;
        break;
      case 'ArrowLeft':
        newIndex = currentIndex - 1;
        break;
      case 'ArrowRight':
        newIndex = currentIndex + 1;
        break;
    }

    if (newIndex >= 0 && newIndex < cells.length) {
      cells[newIndex].focus();
      event.preventDefault();
    }
  }

  // Focus management methods
  focusFirst() {
    this.updateFocusableElements();
    if (this.focusableElements.length > 0) {
      this.focusableElements[0].focus();
      this.currentIndex = 0;
    }
  }

  focusLast() {
    this.updateFocusableElements();
    if (this.focusableElements.length > 0) {
      const lastIndex = this.focusableElements.length - 1;
      this.focusableElements[lastIndex].focus();
      this.currentIndex = lastIndex;
    }
  }

  focusNext() {
    this.updateFocusableElements();
    if (this.focusableElements.length === 0) return;

    this.currentIndex = (this.currentIndex + 1) % this.focusableElements.length;
    this.focusableElements[this.currentIndex].focus();
  }

  focusPrevious() {
    this.updateFocusableElements();
    if (this.focusableElements.length === 0) return;

    this.currentIndex = this.currentIndex <= 0 
      ? this.focusableElements.length - 1 
      : this.currentIndex - 1;
    this.focusableElements[this.currentIndex].focus();
  }

  // Focus trap management
  trapFocus(element: HTMLElement) {
    this.focusTrapStack.push(element);
    
    // Focus the first focusable element in the trap
    const firstFocusable = element.querySelector<HTMLElement>(
      'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
    );
    
    if (firstFocusable) {
      firstFocusable.focus();
    }
  }

  releaseFocusTrap() {
    if (this.focusTrapStack.length > 0) {
      this.focusTrapStack.pop();
    }
  }

  // Utility methods
  private isTextInput(element: HTMLElement): boolean {
    const tagName = element.tagName.toLowerCase();
    if (tagName === 'textarea') return true;
    if (tagName === 'input') {
      const type = (element as HTMLInputElement).type;
      return ['text', 'password', 'email', 'tel', 'url', 'search', 'number'].includes(type);
    }
    return element.contentEditable === 'true';
  }

  private isInGrid(): boolean {
    const activeElement = document.activeElement as HTMLElement;
    return activeElement.closest('[role="grid"]') !== null;
  }

  // Save and restore focus
  saveFocus(): HTMLElement | null {
    return document.activeElement as HTMLElement;
  }

  restoreFocus(element: HTMLElement | null) {
    if (element && element.focus) {
      element.focus();
    }
  }

  // Get current focus information
  getFocusInfo(): {
    element: HTMLElement | null;
    index: number;
    total: number;
  } {
    return {
      element: document.activeElement as HTMLElement,
      index: this.currentIndex,
      total: this.focusableElements.length
    };
  }
}

// Singleton instance
export const focusManager = new FocusManager();

// React hook for focus management
export function useFocusManager(containerRef: React.RefObject<HTMLElement>) {
  React.useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    focusManager.init(container);

    return () => {
      focusManager.destroy();
    };
  }, [containerRef]);

  return {
    focusFirst: () => focusManager.focusFirst(),
    focusLast: () => focusManager.focusLast(),
    focusNext: () => focusManager.focusNext(),
    focusPrevious: () => focusManager.focusPrevious(),
    trapFocus: (element: HTMLElement) => focusManager.trapFocus(element),
    releaseFocusTrap: () => focusManager.releaseFocusTrap(),
    updateFocusableElements: () => focusManager.updateFocusableElements(),
  };
}

// React is not imported, add this import
import React from 'react';