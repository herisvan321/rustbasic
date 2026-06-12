import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Link, usePage } from '@inertiajs/react';
import { useRoute } from '../route';

export interface ThemeColors {
  bg: string;
  text: string;
  headerBg: string;
  headerBorder: string;
  cardBg: string;
  cardBorder: string;
  activeCardBg: string;
  activeCardBorder: string;
  textMain: string;
  textMuted: string;
  orbOpacity: number;
  glowIntensity: string;
  glowIntensityHero: string;
}

export interface ThemeContextType {
  theme: 'dark' | 'light';
  isDark: boolean;
  colors: ThemeColors;
  toggleTheme: () => void;
  isFullscreen: boolean;
  toggleFullscreen: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [theme, setTheme] = useState<'dark' | 'light'>(() => {
    if (typeof window !== 'undefined') {
      return (localStorage.getItem('theme') as 'dark' | 'light') || 'dark';
    }
    return 'dark';
  });

  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(!!document.fullscreenElement);
    };
    document.addEventListener('fullscreenchange', handleFullscreenChange);
    return () => document.removeEventListener('fullscreenchange', handleFullscreenChange);
  }, []);

  useEffect(() => {
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch((err) => {
        console.error(`Error enabling fullscreen: ${err.message}`);
      });
    } else {
      document.exitFullscreen();
    }
  };

  const toggleTheme = () => {
    setTheme(t => (t === 'dark' ? 'light' : 'dark'));
  };

  const isDark = theme === 'dark';
  const colors: ThemeColors = {
    bg: isDark ? '#0a0a0f' : '#f8f9fa',
    text: isDark ? '#e5e5e5' : '#1e293b',
    headerBg: isDark ? 'rgba(10,10,15,0.75)' : 'rgba(248,249,250,0.8)',
    headerBorder: isDark ? 'rgba(232,82,14,0.1)' : 'rgba(232,82,14,0.15)',
    cardBg: isDark ? 'rgba(255,255,255,0.02)' : 'rgba(0,0,0,0.02)',
    cardBorder: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.06)',
    activeCardBg: isDark ? 'rgba(232,82,14,0.03)' : 'rgba(232,82,14,0.04)',
    activeCardBorder: isDark ? 'rgba(232,82,14,0.15)' : 'rgba(232,82,14,0.2)',
    textMain: isDark ? '#fff' : '#0f172a',
    textMuted: isDark ? '#999' : '#475569',
    orbOpacity: isDark ? 0.12 : 0.05,
    glowIntensity: isDark ? 'drop-shadow(0 0 12px rgba(232,82,14,0.4))' : 'drop-shadow(0 0 8px rgba(232,82,14,0.25))',
    glowIntensityHero: isDark ? 'drop-shadow(0 0 24px rgba(232,82,14,0.5))' : 'drop-shadow(0 0 16px rgba(232,82,14,0.3))',
  };

  return (
    <ThemeContext.Provider value={{ theme, isDark, colors, toggleTheme, isFullscreen, toggleFullscreen }}>
      {children}
    </ThemeContext.Provider>
  );
}

interface AppLayoutProps {
  children: ReactNode;
}

function AppLayoutInner({ children }: AppLayoutProps) {
  const { colors, isDark, toggleTheme, isFullscreen, toggleFullscreen } = useTheme();
  const { url } = usePage();
  const currentPath = url.split('?')[0];
  const route = useRoute();

  return (
    <div style={{
      minHeight: '100vh',
      background: colors.bg,
      color: colors.text,
      fontFamily: "'Inter', 'Segoe UI', sans-serif",
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      overflow: 'hidden',
      transition: 'background-color 0.3s, color 0.3s',
    }}>
      {/* Animated Background Orbs */}
      <div style={{
        position: 'absolute', top: '-15%', left: '-10%',
        width: '600px', height: '600px', borderRadius: '50%',
        background: `radial-gradient(circle, rgba(232,82,14,${colors.orbOpacity}) 0%, transparent 70%)`,
        filter: 'blur(80px)', pointerEvents: 'none', zIndex: 0,
        animation: 'float1 8s ease-in-out infinite',
        transition: 'background-color 0.3s',
      }} />
      <div style={{
        position: 'absolute', bottom: '-15%', right: '-10%',
        width: '500px', height: '500px', borderRadius: '50%',
        background: `radial-gradient(circle, rgba(232,82,14,${colors.orbOpacity - 0.04}) 0%, transparent 70%)`,
        filter: 'blur(80px)', pointerEvents: 'none', zIndex: 0,
        animation: 'float2 10s ease-in-out infinite',
        transition: 'background-color 0.3s',
      }} />
      <div style={{
        position: 'absolute', top: '40%', right: '20%',
        width: '300px', height: '300px', borderRadius: '50%',
        background: `radial-gradient(circle, rgba(60,60,60,${isDark ? '0.15' : '0.05'}) 0%, transparent 70%)`,
        filter: 'blur(60px)', pointerEvents: 'none', zIndex: 0,
      }} />

      {/* Subtle Grid Pattern */}
      <div style={{
        position: 'fixed', top: 0, left: 0, width: '100%', height: '100%',
        backgroundImage: isDark 
          ? 'linear-gradient(rgba(232,82,14,0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(232,82,14,0.03) 1px, transparent 1px)'
          : 'linear-gradient(rgba(232,82,14,0.05) 1px, transparent 1px), linear-gradient(90deg, rgba(232,82,14,0.05) 1px, transparent 1px)',
        backgroundSize: '60px 60px',
        pointerEvents: 'none', zIndex: 0,
      }} />

      {/* ===== NAVBAR ===== */}
      <header style={{
        borderBottom: `1px solid ${colors.headerBorder}`,
        position: 'sticky', top: 0, zIndex: 50,
        background: colors.headerBg,
        backdropFilter: 'blur(16px)', WebkitBackdropFilter: 'blur(16px)',
        transition: 'all 0.3s',
      }}>
        <div style={{
          maxWidth: '1280px', margin: '0 auto', padding: '0 24px',
          height: '72px', display: 'flex', alignItems: 'center', justifyContent: 'space-between',
        }}>
          {/* Logo */}
          <Link href={route('home')} style={{ display: 'flex', alignItems: 'center', gap: '12px', textDecoration: 'none' }}>
            <img
              src="/logo.png"
              alt="RustBasic Logo"
              style={{
                width: '42px', height: '42px', objectFit: 'contain',
                filter: colors.glowIntensity,
              }}
            />
            <span style={{ fontWeight: 800, fontSize: '1.25rem', letterSpacing: '-0.02em' }}>
              <span style={{ color: '#e8520e' }}>rust</span>
              <span style={{ color: isDark ? '#8b7355' : '#6b5438' }}>basic</span>
            </span>
          </Link>

          {/* Nav Links */}
          <nav style={{ display: 'flex', alignItems: 'center', gap: '32px' }}>
            <Link
              href={route('home')}
              style={{
                fontSize: '0.875rem', fontWeight: 600, 
                color: currentPath === '/' ? '#e8520e' : (isDark ? '#888' : '#555'),
                textDecoration: 'none', transition: 'color 0.2s',
              }}
            >
              Beranda
            </Link>
            <Link
              href={route('about')}
              style={{
                fontSize: '0.875rem', fontWeight: 600, 
                color: currentPath === '/about' ? '#e8520e' : (isDark ? '#888' : '#555'),
                textDecoration: 'none', transition: 'color 0.2s',
              }}
            >
              Tentang
            </Link>
           
          </nav>

          {/* Controls + Status Badge */}
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            {currentPath !== '/' && (
              <Link
                href={route('home')}
                style={{
                  padding: '8px 20px',
                  borderRadius: '12px',
                  background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                  fontSize: '0.8rem',
                  fontWeight: 700,
                  color: isDark ? '#ccc' : '#444',
                  textDecoration: 'none',
                  border: `1px solid ${colors.cardBorder}`,
                  transition: 'all 0.2s',
                  display: 'inline-flex',
                  alignItems: 'center',
                }}
              >
                ← Kembali
              </Link>
            )}
            {/* Control buttons */}
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '4px',
              borderRadius: '12px',
              background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
              border: `1px solid ${isDark ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.06)'}`,
            }}>
              {/* Theme Toggle Button */}
              <button
                onClick={toggleTheme}
                title={isDark ? "Mode Terang" : "Mode Gelap"}
                style={{
                  display: 'flex', alignItems: 'center', justifyContent: 'center',
                  width: '32px', height: '32px', borderRadius: '8px',
                  background: 'none', border: 'none', cursor: 'pointer',
                  fontSize: '0.95rem',
                  transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => e.currentTarget.style.background = isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.05)'}
                onMouseLeave={(e) => e.currentTarget.style.background = 'none'}
              >
                {isDark ? (
                  <i className="fa-solid fa-sun" style={{ color: '#fbbf24' }}></i>
                ) : (
                  <i className="fa-solid fa-moon" style={{ color: '#475569' }}></i>
                )}
              </button>

              {/* Fullscreen Button */}
              <button
                onClick={toggleFullscreen}
                title={isFullscreen ? "Keluar Layar Penuh" : "Layar Penuh"}
                style={{
                  display: 'flex', alignItems: 'center', justifyContent: 'center',
                  width: '32px', height: '32px', borderRadius: '8px',
                  background: 'none', border: 'none', cursor: 'pointer',
                  fontSize: '0.95rem',
                  transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => e.currentTarget.style.background = isDark ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.05)'}
                onMouseLeave={(e) => e.currentTarget.style.background = 'none'}
              >
                {isFullscreen ? (
                  <i className="fa-solid fa-compress" style={{ color: isDark ? '#ccc' : '#475569' }}></i>
                ) : (
                  <i className="fa-solid fa-expand" style={{ color: isDark ? '#ccc' : '#475569' }}></i>
                )}
              </button>
            </div>

            {/* Status Badge */}
            <div style={{
              display: 'inline-flex', alignItems: 'center', gap: '8px',
              padding: '6px 16px', borderRadius: '999px',
              fontSize: '0.75rem', fontWeight: 700,
              background: 'rgba(16,185,129,0.08)',
              color: '#34d399',
              border: '1px solid rgba(16,185,129,0.15)',
            }}>
              <span style={{
                width: '8px', height: '8px', borderRadius: '50%',
                background: '#34d399',
                boxShadow: '0 0 10px #34d399',
                animation: 'pulse 2s ease-in-out infinite',
              }} />
              Online
            </div>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <div style={{ flexGrow: 1, position: 'relative', zIndex: 10, display: 'flex', flexDirection: 'column' }}>
        {children}
      </div>

      {/* ===== FOOTER ===== */}
      <footer style={{
        borderTop: `1px solid ${isDark ? 'rgba(232,82,14,0.08)' : 'rgba(232,82,14,0.12)'}`,
        padding: '24px 0',
        background: isDark ? 'rgba(10,10,15,0.6)' : 'rgba(248,249,250,0.6)',
        transition: 'all 0.3s',
        position: 'relative',
        zIndex: 10,
      }}>
        <div style={{
          maxWidth: '1280px', margin: '0 auto', padding: '0 24px',
          display: 'flex', flexWrap: 'wrap', alignItems: 'center',
          justifyContent: 'space-between', gap: '16px',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <img src="/logo.png" alt="RustBasic" style={{ width: '24px', height: '24px', objectFit: 'contain' }} />
            <p style={{ margin: 0, fontSize: '0.8rem', color: isDark ? '#555' : '#888' }}>
              © {new Date().getFullYear()} <span style={{ color: '#e8520e' }}>rust</span>
              <span style={{ color: isDark ? '#8b7355' : '#6b5438' }}>basic</span> Framework. Made with ❤️
            </p>
          </div>
          <div style={{ display: 'flex', gap: '24px', alignItems: 'center' }}>
            <a
              href="https://github.com/herisvan321/rustbasic"
              style={{ fontSize: '0.8rem', color: isDark ? '#555' : '#888', textDecoration: 'none', transition: 'color 0.2s' }}
            >
              GitHub
            </a>
            </div>
        </div>
      </footer>

      {/* CSS Animations */}
      <style>{`
        @keyframes float1 {
          0%, 100% { transform: translate(0, 0) scale(1); }
          50% { transform: translate(30px, 20px) scale(1.05); }
        }
        @keyframes float2 {
          0%, 100% { transform: translate(0, 0) scale(1); }
          50% { transform: translate(-20px, -30px) scale(1.08); }
        }
        @keyframes pulse {
          0%, 100% { opacity: 1; transform: scale(1); }
          50% { opacity: 0.5; transform: scale(0.85); }
        }
        @keyframes gradientShift {
          0% { background-position: 0% 50%; }
          50% { background-position: 100% 50%; }
          100% { background-position: 0% 50%; }
        }
        a:hover { color: #e8520e !important; }
        button:hover { transform: translateY(-2px); }
        button:active { transform: scale(0.97) !important; }
      `}</style>
    </div>
  );
}

export default function AppLayout({ children }: AppLayoutProps) {
  return (
    <ThemeProvider>
      <AppLayoutInner>{children}</AppLayoutInner>
    </ThemeProvider>
  );
}
