// Main JavaScript file for Rust Video Editor Demo Site

// DOM elements
const navbar = document.querySelector('.navbar');
const mobileMenuToggle = document.querySelector('.mobile-menu-toggle');
const navMenu = document.querySelector('.nav-menu');
const heroCanvas = document.getElementById('hero-canvas');
const ctx = heroCanvas ? heroCanvas.getContext('2d') : null;

// Initialize on DOM load
document.addEventListener('DOMContentLoaded', () => {
    initNavigation();
    initHeroCanvas();
    initScrollAnimations();
    initPerformanceCharts();
    initDemoPlayers();
    initFormHandlers();
});

// Navigation functionality
function initNavigation() {
    // Mobile menu toggle
    if (mobileMenuToggle) {
        mobileMenuToggle.addEventListener('click', () => {
            mobileMenuToggle.classList.toggle('active');
            navMenu.classList.toggle('mobile-active');
        });
    }

    // Navbar scroll effect
    let lastScroll = 0;
    window.addEventListener('scroll', () => {
        const currentScroll = window.pageYOffset;
        
        if (currentScroll > 50) {
            navbar.classList.add('scrolled');
        } else {
            navbar.classList.remove('scrolled');
        }
        
        lastScroll = currentScroll;
    });

    // Smooth scroll for nav links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const href = this.getAttribute('href');
            // Skip if href is just '#' or empty
            if (!href || href === '#') return;
            
            const target = document.querySelector(href);
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                
                // Close mobile menu if open
                if (navMenu.classList.contains('mobile-active')) {
                    mobileMenuToggle.classList.remove('active');
                    navMenu.classList.remove('mobile-active');
                }
            }
        });
    });
}

// Hero canvas animation
function initHeroCanvas() {
    if (!ctx) return;

    const particles = [];
    const particleCount = 50;
    const connectionDistance = 100;

    // Particle class
    class Particle {
        constructor() {
            this.x = Math.random() * heroCanvas.width;
            this.y = Math.random() * heroCanvas.height;
            this.vx = (Math.random() - 0.5) * 0.5;
            this.vy = (Math.random() - 0.5) * 0.5;
            this.radius = Math.random() * 2 + 1;
        }

        update() {
            this.x += this.vx;
            this.y += this.vy;

            if (this.x < 0 || this.x > heroCanvas.width) this.vx = -this.vx;
            if (this.y < 0 || this.y > heroCanvas.height) this.vy = -this.vy;
        }

        draw() {
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
            ctx.fillStyle = 'rgba(255, 107, 107, 0.8)';
            ctx.fill();
        }
    }

    // Initialize particles
    for (let i = 0; i < particleCount; i++) {
        particles.push(new Particle());
    }

    // Animation loop
    function animate() {
        ctx.clearRect(0, 0, heroCanvas.width, heroCanvas.height);

        // Update and draw particles
        particles.forEach(particle => {
            particle.update();
            particle.draw();
        });

        // Draw connections
        for (let i = 0; i < particles.length; i++) {
            for (let j = i + 1; j < particles.length; j++) {
                const dx = particles[i].x - particles[j].x;
                const dy = particles[i].y - particles[j].y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance < connectionDistance) {
                    ctx.beginPath();
                    ctx.moveTo(particles[i].x, particles[i].y);
                    ctx.lineTo(particles[j].x, particles[j].y);
                    ctx.strokeStyle = `rgba(255, 107, 107, ${1 - distance / connectionDistance})`;
                    ctx.lineWidth = 0.5;
                    ctx.stroke();
                }
            }
        }

        requestAnimationFrame(animate);
    }

    animate();
}

// Scroll animations
function initScrollAnimations() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -100px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('active', 'animated');
                
                // Trigger chart animations
                if (entry.target.classList.contains('chart-wrapper')) {
                    animateCharts();
                }
            }
        });
    }, observerOptions);

    // Observe elements
    document.querySelectorAll('.reveal, .reveal-left, .reveal-right, .feature-card, .demo-card, .testimonial-card, .pricing-card, .chart-wrapper').forEach(el => {
        observer.observe(el);
    });
}

// Performance charts animation
function animateCharts() {
    const bars = document.querySelectorAll('.bar-fill');
    bars.forEach((bar, index) => {
        setTimeout(() => {
            bar.style.width = bar.parentElement.dataset.value + '%';
        }, index * 100);
    });
}

// Initialize performance charts
function initPerformanceCharts() {
    // Set initial width to 0 for animation
    document.querySelectorAll('.bar-fill').forEach(bar => {
        bar.style.width = '0';
        bar.style.transition = 'width 1s ease-out';
    });
}

// Demo video players
function initDemoPlayers() {
    const demoVideos = document.querySelectorAll('.demo-preview video');
    
    demoVideos.forEach(video => {
        // Play on hover
        video.parentElement.addEventListener('mouseenter', () => {
            video.play().catch(e => console.log('Video play failed:', e));
        });
        
        // Pause on leave
        video.parentElement.addEventListener('mouseleave', () => {
            video.pause();
            video.currentTime = 0;
        });
    });
}

// Form handlers
function initFormHandlers() {
    // Pricing buttons
    document.querySelectorAll('.pricing-card .btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.preventDefault();
            const plan = e.target.closest('.pricing-card').querySelector('h3').textContent;
            console.log(`Selected plan: ${plan}`);
            // Handle plan selection
            launchEditor(plan.toLowerCase());
        });
    });
}

// Launch editor function
function launchEditor(plan = 'free') {
    // Add loading state to button
    event.target.classList.add('loading');
    
    // Simulate loading
    setTimeout(() => {
        // In production, this would redirect to the actual editor
        window.location.href = `/workspaces/tcf/rust-video-editor/?plan=${plan}`;
    }, 1000);
}

// Open demo function
function openDemo(type) {
    console.log(`Opening ${type} demo`);
    // In production, this would open the specific demo
    window.location.href = `/workspaces/tcf/rust-video-editor/?demo=${type}`;
}

// Stats counter animation
function animateStats() {
    const stats = document.querySelectorAll('.stat-value');
    
    stats.forEach(stat => {
        const target = stat.textContent;
        const isNumeric = !isNaN(target);
        
        if (isNumeric) {
            const finalValue = parseInt(target);
            let currentValue = 0;
            const increment = finalValue / 50;
            
            const counter = setInterval(() => {
                currentValue += increment;
                if (currentValue >= finalValue) {
                    currentValue = finalValue;
                    clearInterval(counter);
                }
                stat.textContent = Math.floor(currentValue);
            }, 30);
        }
    });
}

// Trigger stats animation when in view
const statsObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            animateStats();
            statsObserver.unobserve(entry.target);
        }
    });
});

const heroStats = document.querySelector('.hero-stats');
if (heroStats) {
    statsObserver.observe(heroStats);
}

// Add parallax scrolling effect
window.addEventListener('scroll', () => {
    const scrolled = window.pageYOffset;
    const parallaxElements = document.querySelectorAll('.parallax');
    
    parallaxElements.forEach(element => {
        const speed = element.dataset.speed || 0.5;
        element.style.transform = `translateY(${scrolled * speed}px)`;
    });
});

// Handle video loading errors
document.querySelectorAll('video').forEach(video => {
    video.addEventListener('error', (e) => {
        console.error('Video loading error:', e);
        // Show placeholder or fallback content
        const placeholder = document.createElement('div');
        placeholder.className = 'video-placeholder';
        placeholder.textContent = 'Demo video coming soon';
        video.parentElement.appendChild(placeholder);
        video.style.display = 'none';
    });
});

// Lazy load images
const imageObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const img = entry.target;
            img.src = img.dataset.src;
            img.classList.add('loaded');
            imageObserver.unobserve(img);
        }
    });
});

document.querySelectorAll('img[data-src]').forEach(img => {
    imageObserver.observe(img);
});

// Add keyboard navigation
document.addEventListener('keydown', (e) => {
    // ESC key closes mobile menu
    if (e.key === 'Escape' && navMenu.classList.contains('mobile-active')) {
        mobileMenuToggle.classList.remove('active');
        navMenu.classList.remove('mobile-active');
    }
});

// Performance optimization: Debounce scroll events
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Apply debouncing to scroll handlers
const debouncedScroll = debounce(() => {
    // Handle scroll-based animations
}, 100);

window.addEventListener('scroll', debouncedScroll);

// Initialize tooltips
function initTooltips() {
    const tooltips = document.querySelectorAll('[data-tooltip]');
    
    tooltips.forEach(element => {
        const tooltip = document.createElement('div');
        tooltip.className = 'tooltip';
        tooltip.textContent = element.dataset.tooltip;
        
        element.addEventListener('mouseenter', () => {
            document.body.appendChild(tooltip);
            const rect = element.getBoundingClientRect();
            tooltip.style.top = rect.top - tooltip.offsetHeight - 10 + 'px';
            tooltip.style.left = rect.left + (rect.width - tooltip.offsetWidth) / 2 + 'px';
            tooltip.classList.add('visible');
        });
        
        element.addEventListener('mouseleave', () => {
            tooltip.classList.remove('visible');
            setTimeout(() => tooltip.remove(), 300);
        });
    });
}

initTooltips();

// Export functions for use in other scripts
window.rustVideoEditor = {
    launchEditor,
    openDemo,
    animateStats,
    animateCharts
};