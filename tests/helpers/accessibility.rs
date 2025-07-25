//! Accessibility testing utilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WCAG compliance levels
#[derive(Debug, Clone, PartialEq)]
pub enum WcagLevel {
    A,
    AA,
    AAA,
}

/// Accessibility violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityViolation {
    pub rule: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub element: String,
    pub help_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Minor,
    Moderate,
    Serious,
    Critical,
}

/// Accessibility test result
#[derive(Debug, Clone)]
pub struct AccessibilityResult {
    pub violations: Vec<AccessibilityViolation>,
    pub passes: Vec<String>,
    pub wcag_level: WcagLevel,
}

impl AccessibilityResult {
    pub fn is_compliant(&self) -> bool {
        self.violations.is_empty()
    }
    
    pub fn critical_violations(&self) -> Vec<&AccessibilityViolation> {
        self.violations
            .iter()
            .filter(|v| v.impact == ImpactLevel::Critical)
            .collect()
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("Accessibility Test Report (WCAG {:?})\n", self.wcag_level));
        report.push_str(&format!("=").__repeat(50).__add__("\n\n"));
        
        if self.violations.is_empty() {
            report.push_str("✅ No accessibility violations found!\n");
        } else {
            report.push_str(&format!("❌ Found {} violations:\n\n", self.violations.len()));
            
            for (i, violation) in self.violations.iter().enumerate() {
                report.push_str(&format!("{}. {} ({:?})\n", i + 1, violation.rule, violation.impact));
                report.push_str(&format!("   Description: {}\n", violation.description));
                report.push_str(&format!("   Element: {}\n", violation.element));
                if let Some(url) = &violation.help_url {
                    report.push_str(&format!("   Help: {}\n", url));
                }
                report.push_str("\n");
            }
        }
        
        report.push_str(&format!("\n✅ {} checks passed\n", self.passes.len()));
        
        report
    }
}

/// Color contrast checker
pub mod contrast {
    /// Calculate relative luminance of a color
    pub fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
        let r = srgb_to_linear(r as f64 / 255.0);
        let g = srgb_to_linear(g as f64 / 255.0);
        let b = srgb_to_linear(b as f64 / 255.0);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    
    fn srgb_to_linear(channel: f64) -> f64 {
        if channel <= 0.03928 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }
    
    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(
        r1: u8, g1: u8, b1: u8,
        r2: u8, g2: u8, b2: u8,
    ) -> f64 {
        let l1 = relative_luminance(r1, g1, b1);
        let l2 = relative_luminance(r2, g2, b2);
        
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        
        (lighter + 0.05) / (darker + 0.05)
    }
    
    /// Check if contrast meets WCAG requirements
    pub fn meets_wcag(
        ratio: f64,
        level: super::WcagLevel,
        large_text: bool,
    ) -> bool {
        match (level, large_text) {
            (super::WcagLevel::AA, false) => ratio >= 4.5,
            (super::WcagLevel::AA, true) => ratio >= 3.0,
            (super::WcagLevel::AAA, false) => ratio >= 7.0,
            (super::WcagLevel::AAA, true) => ratio >= 4.5,
            (super::WcagLevel::A, _) => ratio >= 3.0,
        }
    }
}

/// Keyboard navigation tester
pub struct KeyboardNavigationTester {
    tab_order: Vec<String>,
    focus_trapped: bool,
    shortcuts: HashMap<String, String>,
}

impl KeyboardNavigationTester {
    pub fn new() -> Self {
        Self {
            tab_order: Vec::new(),
            focus_trapped: false,
            shortcuts: HashMap::new(),
        }
    }
    
    pub fn record_tab(&mut self, element: String) {
        self.tab_order.push(element);
    }
    
    pub fn check_tab_order(&self) -> Result<(), String> {
        // Check for logical tab order
        if self.tab_order.is_empty() {
            return Err("No focusable elements found".to_string());
        }
        
        // Check for focus trap
        if self.focus_trapped {
            return Err("Focus trap detected".to_string());
        }
        
        Ok(())
    }
    
    pub fn add_shortcut(&mut self, key: String, action: String) {
        self.shortcuts.insert(key, action);
    }
    
    pub fn check_shortcuts(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        // Check for conflicts with browser shortcuts
        let browser_shortcuts = ["Ctrl+S", "Ctrl+P", "Ctrl+F", "F5", "F11"];
        
        for shortcut in &browser_shortcuts {
            if self.shortcuts.contains_key(*shortcut) {
                issues.push(format!("Conflicts with browser shortcut: {}", shortcut));
            }
        }
        
        // Check for proper modifier keys
        for (key, _) in &self.shortcuts {
            if !key.contains("Ctrl") && !key.contains("Alt") && !key.contains("Shift") {
                if key.len() == 1 {
                    issues.push(format!("Single key shortcut without modifier: {}", key));
                }
            }
        }
        
        issues
    }
}

/// Screen reader simulator
pub struct ScreenReaderSimulator {
    announcements: Vec<String>,
    current_focus: Option<String>,
}

impl ScreenReaderSimulator {
    pub fn new() -> Self {
        Self {
            announcements: Vec::new(),
            current_focus: None,
        }
    }
    
    pub fn announce(&mut self, text: String) {
        self.announcements.push(text);
    }
    
    pub fn focus(&mut self, element: String) {
        self.current_focus = Some(element.clone());
        self.announce(format!("Focus moved to {}", element));
    }
    
    pub fn get_transcript(&self) -> Vec<String> {
        self.announcements.clone()
    }
    
    pub fn check_announcements(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        for announcement in &self.announcements {
            // Check for missing context
            if announcement.contains("button") && !announcement.contains("button,") {
                issues.push(format!("Button without purpose: {}", announcement));
            }
            
            // Check for redundant announcements
            if announcement.contains("image image") || announcement.contains("button button") {
                issues.push(format!("Redundant announcement: {}", announcement));
            }
            
            // Check for missing labels
            if announcement == "unlabeled" || announcement.is_empty() {
                issues.push("Unlabeled element found".to_string());
            }
        }
        
        issues
    }
}

/// Visual indicator tester
pub struct VisualIndicatorTester {
    focus_indicators: HashMap<String, bool>,
    hover_indicators: HashMap<String, bool>,
}

impl VisualIndicatorTester {
    pub fn new() -> Self {
        Self {
            focus_indicators: HashMap::new(),
            hover_indicators: HashMap::new(),
        }
    }
    
    pub fn record_focus_indicator(&mut self, element: String, visible: bool) {
        self.focus_indicators.insert(element, visible);
    }
    
    pub fn record_hover_indicator(&mut self, element: String, visible: bool) {
        self.hover_indicators.insert(element, visible);
    }
    
    pub fn check_indicators(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        for (element, visible) in &self.focus_indicators {
            if !visible {
                issues.push(format!("Missing focus indicator: {}", element));
            }
        }
        
        issues
    }
}

/// ARIA attribute validator
pub fn validate_aria_attributes(
    element: &str,
    role: Option<&str>,
    attributes: &HashMap<String, String>,
) -> Vec<String> {
    let mut issues = Vec::new();
    
    // Check role validity
    if let Some(role) = role {
        let valid_roles = [
            "button", "link", "navigation", "main", "banner",
            "contentinfo", "form", "search", "region", "article",
            "complementary", "dialog", "alert", "status", "progressbar",
            "menu", "menubar", "menuitem", "tab", "tabpanel", "tablist",
        ];
        
        if !valid_roles.contains(&role) {
            issues.push(format!("Invalid ARIA role: {}", role));
        }
    }
    
    // Check required attributes for roles
    match role {
        Some("progressbar") => {
            if !attributes.contains_key("aria-valuenow") {
                issues.push("progressbar missing aria-valuenow".to_string());
            }
        }
        Some("slider") => {
            if !attributes.contains_key("aria-valuenow") ||
               !attributes.contains_key("aria-valuemin") ||
               !attributes.contains_key("aria-valuemax") {
                issues.push("slider missing required aria-value attributes".to_string());
            }
        }
        _ => {}
    }
    
    // Check for proper labeling
    if !attributes.contains_key("aria-label") && 
       !attributes.contains_key("aria-labelledby") &&
       element != "img" { // Images can use alt
        issues.push(format!("{} missing accessible label", element));
    }
    
    issues
}

/// Test macros for accessibility
#[macro_export]
macro_rules! assert_accessible {
    ($element:expr) => {
        let result = $crate::helpers::accessibility::test_element_accessibility($element);
        assert!(
            result.is_compliant(),
            "Accessibility violations found: {:?}",
            result.violations
        );
    };
}

#[macro_export]
macro_rules! assert_contrast_ratio {
    ($fg:expr, $bg:expr, $level:expr) => {
        let ratio = $crate::helpers::accessibility::contrast::contrast_ratio(
            $fg.0, $fg.1, $fg.2,
            $bg.0, $bg.1, $bg.2
        );
        assert!(
            $crate::helpers::accessibility::contrast::meets_wcag(ratio, $level, false),
            "Contrast ratio {:.2} does not meet {:?} requirements",
            ratio, $level
        );
    };
}