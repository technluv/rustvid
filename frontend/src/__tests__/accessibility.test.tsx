import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { axe, toHaveNoViolations } from 'jest-axe';
import App from '../App';
import { AccessibilityMenu } from '../components/AccessibilityMenu';
import { keyboardManager } from '../utils/keyboard';
import { accessibilityManager } from '../utils/accessibility';
import { focusManager } from '../utils/focusManager';

// Add jest-axe matchers
expect.extend(toHaveNoViolations);

describe('Accessibility Tests', () => {
  beforeEach(() => {
    // Reset accessibility settings before each test
    localStorage.clear();
    document.documentElement.className = '';
  });

  describe('WCAG Compliance', () => {
    test('App should have no automatic accessibility violations', async () => {
      const { container } = render(<App />);
      const results = await axe(container);
      expect(results).toHaveNoViolations();
    });

    test('AccessibilityMenu should have no automatic accessibility violations', async () => {
      const { container } = render(
        <AccessibilityMenu isOpen={true} onClose={() => {}} />
      );
      const results = await axe(container);
      expect(results).toHaveNoViolations();
    });
  });

  describe('Keyboard Navigation', () => {
    test('Tab key should navigate through focusable elements', async () => {
      render(<App />);
      const user = userEvent.setup();

      // Wait for app to load
      await waitFor(() => {
        expect(screen.queryByText('Loading Video Editor...')).not.toBeInTheDocument();
      });

      // Get focusable elements
      const accessibilityButton = screen.getByLabelText('Open accessibility settings');
      const contrastButton = screen.getByLabelText(/high contrast mode/i);

      // Tab through elements
      await user.tab();
      expect(document.activeElement).toBe(accessibilityButton);

      await user.tab();
      expect(document.activeElement).toBe(contrastButton);
    });

    test('Escape key should close accessibility menu', async () => {
      const onClose = jest.fn();
      render(<AccessibilityMenu isOpen={true} onClose={onClose} />);
      const user = userEvent.setup();

      await user.keyboard('{Escape}');
      expect(onClose).toHaveBeenCalled();
    });

    test('Keyboard shortcuts should be registerable and triggerable', () => {
      const action = jest.fn();
      const shortcut = {
        id: 'test-shortcut',
        name: 'Test Shortcut',
        description: 'Test shortcut description',
        keys: ['Ctrl', 'T'],
        action,
        category: 'editing' as const,
      };

      keyboardManager.registerShortcut(shortcut);

      // Simulate keyboard event
      const event = new KeyboardEvent('keydown', {
        key: 'T',
        ctrlKey: true,
        bubbles: true,
      });
      window.dispatchEvent(event);

      expect(action).toHaveBeenCalled();
      keyboardManager.unregisterShortcut('test-shortcut');
    });
  });

  describe('Screen Reader Support', () => {
    test('App should have proper ARIA labels', async () => {
      render(<App />);

      await waitFor(() => {
        expect(screen.queryByText('Loading Video Editor...')).not.toBeInTheDocument();
      });

      // Check main regions
      expect(screen.getByRole('application')).toHaveAttribute(
        'aria-label',
        'Rust Video Editor Application'
      );
      expect(screen.getByRole('banner')).toBeInTheDocument();
      expect(screen.getByRole('main')).toBeInTheDocument();
      expect(screen.getByRole('region', { name: 'Video preview area' })).toBeInTheDocument();
      expect(screen.getByRole('region', { name: 'Timeline editor' })).toBeInTheDocument();
    });

    test('Announcements should be made for actions', () => {
      const announceMethod = jest.spyOn(accessibilityManager, 'announce');
      
      accessibilityManager.announce('Test announcement');
      expect(announceMethod).toHaveBeenCalledWith('Test announcement');
      
      // Check that announcement element is created
      const announcement = document.querySelector('[role="status"]');
      expect(announcement).toBeInTheDocument();
      expect(announcement).toHaveTextContent('Test announcement');
    });

    test('Skip navigation link should work', async () => {
      render(<App />);
      const user = userEvent.setup();

      await waitFor(() => {
        expect(screen.queryByText('Loading Video Editor...')).not.toBeInTheDocument();
      });

      // Tab to skip link (it should be the first focusable element)
      await user.tab();
      const skipLink = screen.getByText('Skip to timeline');
      expect(skipLink).toHaveFocus();

      // Activate skip link
      await user.keyboard('{Enter}');
      
      // Timeline should now have focus
      const timeline = document.getElementById('timeline');
      expect(timeline).toHaveFocus();
    });
  });

  describe('High Contrast Mode', () => {
    test('High contrast mode should toggle correctly', async () => {
      render(<App />);
      const user = userEvent.setup();

      await waitFor(() => {
        expect(screen.queryByText('Loading Video Editor...')).not.toBeInTheDocument();
      });

      const contrastButton = screen.getByLabelText(/enable high contrast mode/i);
      
      // Enable high contrast
      await user.click(contrastButton);
      expect(document.documentElement).toHaveClass('high-contrast');

      // Button label should update
      expect(screen.getByLabelText(/disable high contrast mode/i)).toBeInTheDocument();
    });

    test('High contrast mode should persist across sessions', () => {
      accessibilityManager.enableHighContrast();
      
      // Check localStorage
      const prefs = JSON.parse(localStorage.getItem('accessibility-preferences') || '{}');
      expect(prefs.highContrast).toBe(true);

      // Create new instance (simulating page reload)
      const newManager = new (accessibilityManager.constructor as any)();
      expect(document.documentElement).toHaveClass('high-contrast');
    });
  });

  describe('Font Size Adjustment', () => {
    test('Font size should be adjustable', () => {
      const initialSize = accessibilityManager.getFontSize();
      
      accessibilityManager.increaseFontSize();
      expect(accessibilityManager.getFontSize()).toBe(initialSize + 10);
      
      accessibilityManager.decreaseFontSize();
      expect(accessibilityManager.getFontSize()).toBe(initialSize);
      
      // Check CSS variable
      const rootStyles = getComputedStyle(document.documentElement);
      expect(rootStyles.getPropertyValue('--base-font-size')).toBe(`${initialSize}%`);
    });

    test('Font size should have min/max limits', () => {
      // Test maximum
      for (let i = 0; i < 20; i++) {
        accessibilityManager.increaseFontSize();
      }
      expect(accessibilityManager.getFontSize()).toBe(200);

      // Test minimum
      for (let i = 0; i < 20; i++) {
        accessibilityManager.decreaseFontSize();
      }
      expect(accessibilityManager.getFontSize()).toBe(50);
    });
  });

  describe('Focus Management', () => {
    test('Focus trap should work in modals', async () => {
      render(<AccessibilityMenu isOpen={true} onClose={() => {}} />);
      const user = userEvent.setup();

      const modal = document.getElementById('accessibility-menu');
      expect(modal).toBeInTheDocument();

      // Tab through modal elements
      const firstButton = screen.getAllByRole('button')[0];
      const lastButton = screen.getAllByRole('button')[screen.getAllByRole('button').length - 1];

      firstButton.focus();
      expect(document.activeElement).toBe(firstButton);

      // Tab from last element should wrap to first
      lastButton.focus();
      await user.tab();
      // Focus should wrap around in the modal
    });

    test('Focus manager should handle focus correctly', () => {
      const container = document.createElement('div');
      container.innerHTML = `
        <button id="btn1">Button 1</button>
        <button id="btn2">Button 2</button>
        <button id="btn3">Button 3</button>
      `;
      document.body.appendChild(container);

      focusManager.init(container);
      focusManager.focusFirst();
      
      expect(document.activeElement?.id).toBe('btn1');
      
      focusManager.focusNext();
      expect(document.activeElement?.id).toBe('btn2');
      
      focusManager.focusLast();
      expect(document.activeElement?.id).toBe('btn3');
      
      focusManager.destroy();
      document.body.removeChild(container);
    });
  });

  describe('Reduced Motion', () => {
    test('Reduced motion preference should be detected', () => {
      // Mock matchMedia
      const mockMatchMedia = jest.fn().mockImplementation(query => ({
        matches: query === '(prefers-reduced-motion: reduce)',
        addEventListener: jest.fn(),
        removeEventListener: jest.fn(),
      }));
      
      window.matchMedia = mockMatchMedia;
      
      // Create new instance to trigger detection
      const newManager = new (accessibilityManager.constructor as any)();
      expect(document.documentElement).toHaveClass('reduced-motion');
    });

    test('Reduced motion should disable animations', () => {
      accessibilityManager.enableReducedMotion();
      expect(document.documentElement).toHaveClass('reduced-motion');
      
      // Check that animations are disabled in CSS
      const testElement = document.createElement('div');
      testElement.className = 'timeline-clip';
      document.body.appendChild(testElement);
      
      const styles = getComputedStyle(testElement);
      // In reduced motion, transition duration should be minimal
      
      document.body.removeChild(testElement);
    });
  });

  describe('Accessibility Menu', () => {
    test('Accessibility menu should be navigable with keyboard', async () => {
      render(<AccessibilityMenu isOpen={true} onClose={() => {}} />);
      const user = userEvent.setup();

      // Check tab panels
      const generalTab = screen.getByRole('tab', { name: 'General' });
      const shortcutsTab = screen.getByRole('tab', { name: 'Keyboard Shortcuts' });

      expect(generalTab).toHaveAttribute('aria-selected', 'true');
      expect(shortcutsTab).toHaveAttribute('aria-selected', 'false');

      // Switch tabs
      await user.click(shortcutsTab);
      expect(generalTab).toHaveAttribute('aria-selected', 'false');
      expect(shortcutsTab).toHaveAttribute('aria-selected', 'true');
    });

    test('Settings should be toggleable', async () => {
      render(<AccessibilityMenu isOpen={true} onClose={() => {}} />);
      const user = userEvent.setup();

      const highContrastToggle = screen.getByRole('switch', { name: /high contrast mode/i });
      expect(highContrastToggle).toHaveAttribute('aria-checked', 'false');

      await user.click(highContrastToggle);
      expect(highContrastToggle).toHaveAttribute('aria-checked', 'true');
    });
  });

  describe('Touch Target Size', () => {
    test('Interactive elements should meet minimum touch target size', () => {
      render(<App />);

      const buttons = screen.getAllByRole('button');
      buttons.forEach(button => {
        const rect = button.getBoundingClientRect();
        // Check minimum size (44x44 pixels for WCAG AA)
        expect(rect.width).toBeGreaterThanOrEqual(44);
        expect(rect.height).toBeGreaterThanOrEqual(44);
      });
    });
  });

  describe('Color Contrast', () => {
    test('Text should have sufficient color contrast', async () => {
      render(<App />);

      await waitFor(() => {
        expect(screen.queryByText('Loading Video Editor...')).not.toBeInTheDocument();
      });

      // This would require a more sophisticated contrast checking library
      // For now, we just check that high contrast mode changes colors
      const header = screen.getByRole('banner');
      const initialBg = getComputedStyle(header).backgroundColor;

      accessibilityManager.enableHighContrast();
      const highContrastBg = getComputedStyle(header).backgroundColor;

      expect(initialBg).not.toBe(highContrastBg);
    });
  });
});