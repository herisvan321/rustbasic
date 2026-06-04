import { useState, useEffect } from 'react';
import { Link, Head } from '@inertiajs/react';
import { useTheme } from '../Layouts/AppLayout';

interface WelcomeProps {
  title?: string;
  auth_installed?: boolean;
  is_logged_in?: boolean;
}

export default function Welcome({ title, auth_installed, is_logged_in }: WelcomeProps) {
  const [count, setCount] = useState(0);
  const [mounted, setMounted] = useState(false);
  const { colors, isDark } = useTheme();

  useEffect(() => {
    setMounted(true);
  }, []);

  return (
    <>
      <Head>
        <title>{title || "RustBasic - Full-stack Rust Web Framework"}</title>
        <meta name="description" content="RustBasic adalah framework web full-stack modern berkinerja tinggi ditenagai Rust (Custom HTTP Engine) & React.js melalui Inertia.js." />
        <meta name="keywords" content="rust, rustbasic, web framework, react, inertia, spa, web development" />
        <meta property="og:title" content="RustBasic - Full-stack Rust Web Framework" />
        <meta property="og:description" content="Bangun web modern secepat kilat dengan backend Rust dan frontend React." />
        <meta property="og:image" content="/logo.png" />
        <meta name="twitter:card" content="summary_large_image" />
      </Head>

      {/* ===== HERO SECTION ===== */}
      <main style={{
        maxWidth: '1280px',
        margin: '0 auto',
        padding: '80px 24px 60px',
        flexGrow: 1,
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        position: 'relative',
        zIndex: 10,
      }}>
        <div style={{
          display: 'flex',
          flexWrap: 'wrap',
          gap: '64px',
          alignItems: 'center',
        }}>
          {/* Left: Hero Content */}
          <div style={{ flex: '1 1 500px', display: 'flex', flexDirection: 'column', gap: '24px' }}>
            {/* Logo + Tagline Badge */}
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '16px',
              opacity: mounted ? 1 : 0,
              transform: mounted ? 'translateY(0)' : 'translateY(20px)',
              transition: 'all 0.6s ease',
            }}>
              <img
                src="/logo.png"
                alt="RustBasic"
                style={{
                  width: '80px',
                  height: '80px',
                  objectFit: 'contain',
                  filter: colors.glowIntensityHero,
                }}
              />
              <div style={{
                display: 'inline-flex',
                padding: '8px 20px',
                borderRadius: '999px',
                fontSize: '0.8rem',
                fontWeight: 700,
                letterSpacing: '0.05em',
                background: isDark
                  ? 'linear-gradient(135deg, rgba(232,82,14,0.12), rgba(139,115,85,0.08))'
                  : 'linear-gradient(135deg, rgba(232,82,14,0.08), rgba(139,115,85,0.05))',
                border: `1px solid ${isDark ? 'rgba(232,82,14,0.2)' : 'rgba(232,82,14,0.15)'}`,
                color: '#e8520e',
              }}>
                ⚡ Full-stack Rust Framework
              </div>
            </div>

            {/* Main Headline */}
            <h1 style={{
              fontSize: 'clamp(2.5rem, 5vw, 3.8rem)',
              fontWeight: 900,
              letterSpacing: '-0.03em',
              lineHeight: 1.1,
              margin: 0,
              color: colors.textMain,
              opacity: mounted ? 1 : 0,
              transform: mounted ? 'translateY(0)' : 'translateY(20px)',
              transition: 'all 0.6s ease 0.1s, color 0.3s',
            }}>
              Bangun Web Modern{' '}
              <span style={{
                background: 'linear-gradient(135deg, #e8520e, #ff8c42, #e8520e)',
                backgroundSize: '200% 200%',
                WebkitBackgroundClip: 'text',
                backgroundClip: 'text',
                color: 'transparent',
                animation: 'gradientShift 4s ease infinite',
              }}>
                Secepat Kilat
              </span>
            </h1>
             {/* Subtitle */}
            <p style={{
              fontSize: '1.15rem',
              lineHeight: 1.7,
              color: colors.textMuted,
              margin: 0,
              maxWidth: '560px',
              opacity: mounted ? 1 : 0,
              transform: mounted ? 'translateY(0)' : 'translateY(20px)',
              transition: 'all 0.6s ease 0.2s, color 0.3s',
            }}>
              Backend <b style={{ color: isDark ? '#ccc' : '#444' }}>Rust</b> berkinerja tinggi bersatu
              dengan <b style={{ color: isDark ? '#ccc' : '#444' }}>React</b> via jembatan{' '}
              <b style={{ color: isDark ? '#ccc' : '#444' }}>Inertia.js</b>.
              Satu codebase, tanpa API terpisah, performa luar biasa.
            </p>

            {/* CTA Buttons */}
            <div style={{
              display: 'flex',
              flexWrap: 'wrap',
              gap: '16px',
              marginTop: '8px',
              opacity: mounted ? 1 : 0,
              transform: mounted ? 'translateY(0)' : 'translateY(20px)',
              transition: 'all 0.6s ease 0.3s',
            }}>
              <Link
                href="/about"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: '#fff',
                  textDecoration: 'none',
                  boxShadow: '0 8px 32px rgba(232,82,14,0.35), inset 0 1px 0 rgba(255,255,255,0.15)',
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                Jelajahi SPA →
              </Link>
              {auth_installed && (
                is_logged_in ? (
                  <Link
                    href="/dashboard"
                    style={{
                      padding: '16px 32px',
                      borderRadius: '16px',
                      background: 'linear-gradient(135deg, #4f46e5, #4338ca)',
                      fontWeight: 700,
                      fontSize: '0.95rem',
                      color: '#fff',
                      textDecoration: 'none',
                      boxShadow: '0 8px 32px rgba(79,70,229,0.2)',
                      transition: 'all 0.3s ease',
                      display: 'inline-flex',
                      alignItems: 'center',
                    }}
                  >
                    Dashboard
                  </Link>
                ) : (
                  <>
                    <Link
                      href="/login"
                      style={{
                        padding: '16px 32px',
                        borderRadius: '16px',
                        background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                        fontWeight: 700,
                        fontSize: '0.95rem',
                        color: isDark ? '#ccc' : '#444',
                        textDecoration: 'none',
                        border: `1px solid ${isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}`,
                        transition: 'all 0.3s ease',
                        display: 'inline-flex',
                        alignItems: 'center',
                      }}
                    >
                      Masuk
                    </Link>
                    <Link
                      href="/register"
                      style={{
                        padding: '16px 32px',
                        borderRadius: '16px',
                        background: 'linear-gradient(135deg, #4f46e5, #4338ca)',
                        fontWeight: 700,
                        fontSize: '0.95rem',
                        color: '#fff',
                        textDecoration: 'none',
                        boxShadow: '0 8px 32px rgba(79,70,229,0.2)',
                        transition: 'all 0.3s ease',
                        display: 'inline-flex',
                        alignItems: 'center',
                      }}
                    >
                      Daftar
                    </Link>
                  </>
                )
              )}
              <a
                href="/dev"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: isDark ? '#ccc' : '#444',
                  textDecoration: 'none',
                  border: `1px solid ${isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}`,
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                }}
              >
                Cek JSON Config
              </a>
            </div>

            {/* Tech Stack */}
            <div style={{
              display: 'flex',
              flexWrap: 'wrap',
              alignItems: 'center',
              gap: '12px',
              marginTop: '16px',
              opacity: mounted ? 1 : 0,
              transition: 'all 0.6s ease 0.4s',
            }}>
              <span style={{
                fontSize: '0.7rem',
                fontWeight: 700,
                color: isDark ? '#555' : '#888',
                letterSpacing: '0.1em',
                textTransform: 'uppercase',
              }}>
                Powered by
              </span>
              {['Rust', 'React', 'Vite', 'Inertia.js'].map((tech) => (
                <span key={tech} style={{
                  padding: '6px 14px',
                  borderRadius: '8px',
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                  color: isDark ? '#aaa' : '#444',
                  border: `1px solid ${isDark ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.06)'}`,
                }}>
                  {tech}
                </span>
              ))}
            </div>
          </div>

          {/* Right: Interactive Widgets */}
          <div style={{
            width: '100%',
            maxWidth: '400px',
            display: 'flex',
            flexDirection: 'column',
            gap: '24px',
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateX(0)' : 'translateX(40px)',
            transition: 'all 0.8s ease 0.3s',
          }}>
            {/* React Counter Card */}
            <div style={{
              padding: '32px',
              borderRadius: '24px',
              border: `1px solid ${colors.activeCardBorder}`,
              background: colors.activeCardBg,
              backdropFilter: 'blur(20px)',
              position: 'relative',
              overflow: 'hidden',
              transition: 'background-color 0.3s, border-color 0.3s',
            }}>
              {/* Decorative glow */}
              <div style={{
                position: 'absolute',
                top: '-20px',
                right: '-20px',
                width: '100px',
                height: '100px',
                borderRadius: '50%',
                background: 'rgba(232,82,14,0.1)',
                filter: 'blur(30px)',
                pointerEvents: 'none',
              }} />

              <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                marginBottom: '20px',
              }}>
                <span style={{
                  fontSize: '0.7rem',
                  fontWeight: 700,
                  color: '#e8520e',
                  letterSpacing: '0.1em',
                  textTransform: 'uppercase',
                }}>
                  Reaktifitas React
                </span>
                <span style={{
                  width: '10px',
                  height: '10px',
                  borderRadius: '50%',
                  background: '#e8520e',
                  boxShadow: '0 0 12px #e8520e',
                  animation: 'pulse 2s ease-in-out infinite',
                }} />
              </div>

              <h3 style={{
                fontSize: '1.25rem',
                fontWeight: 800,
                color: colors.textMain,
                margin: '0 0 8px 0',
                transition: 'color 0.3s',
              }}>
                Uji State Counter
              </h3>
              <p style={{
                fontSize: '0.85rem',
                color: colors.textMuted,
                lineHeight: 1.6,
                margin: '0 0 24px 0',
                transition: 'color 0.3s',
              }}>
                Klik tombol untuk melihat performa client-side React tanpa server round-trip.
              </p>

              <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
                <button
                  onClick={() => setCount(c => c + 1)}
                  style={{
                    padding: '14px 28px',
                    borderRadius: '14px',
                    background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                    fontWeight: 700,
                    fontSize: '0.9rem',
                    color: '#fff',
                    border: 'none',
                    cursor: 'pointer',
                    boxShadow: '0 4px 20px rgba(232,82,14,0.3)',
                    transition: 'all 0.2s ease',
                  }}
                >
                  Tambah: {count}
                </button>
                <button
                  onClick={() => setCount(0)}
                  style={{
                    background: 'none',
                    border: 'none',
                    color: isDark ? '#666' : '#888',
                    fontWeight: 600,
                    fontSize: '0.85rem',
                    cursor: 'pointer',
                    transition: 'color 0.2s',
                  }}
                >
                  Reset
                </button>
              </div>
            </div>

            {/* Integration Card */}
            <div style={{
              padding: '32px',
              borderRadius: '24px',
              border: `1px solid ${colors.cardBorder}`,
              background: colors.cardBg,
              position: 'relative',
              overflow: 'hidden',
              transition: 'background-color 0.3s, border-color 0.3s',
            }}>
              <h3 style={{
                fontSize: '1.1rem',
                fontWeight: 800,
                color: colors.textMain,
                margin: '0 0 12px 0',
                transition: 'color 0.3s',
              }}>
                ⚙️ Integrasi 100% Terhubung
              </h3>
              <p style={{
                fontSize: '0.85rem',
                color: colors.textMuted,
                lineHeight: 1.6,
                margin: '0 0 16px 0',
                transition: 'color 0.3s',
              }}>
                Backend Rust dan Frontend React <b style={{ color: isDark ? '#ccc' : '#444' }}>saling terikat erat</b> (monolith)
                namun modular. Cepat dikembangkan, luar biasa saat di-serve!
              </p>

              <div style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                fontSize: '0.75rem',
                fontWeight: 700,
              }}>
                <span style={{ color: isDark ? '#8b7355' : '#6b5438' }}>Auth Scaffold:</span>
                <span style={{
                  padding: '4px 12px',
                  borderRadius: '6px',
                  background: auth_installed ? 'rgba(16,185,129,0.1)' : 'rgba(245,158,11,0.1)',
                  color: auth_installed ? '#34d399' : '#fbbf24',
                  border: `1px solid ${auth_installed ? 'rgba(16,185,129,0.2)' : 'rgba(245,158,11,0.2)'}`,
                }}>
                  {auth_installed ? '✓ Installed' : 'Belum Terinstal'}
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Feature Highlights Section */}
        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
          gap: '20px',
          marginTop: '80px',
          opacity: mounted ? 1 : 0,
          transform: mounted ? 'translateY(0)' : 'translateY(30px)',
          transition: 'all 0.8s ease 0.5s',
        }}>
          {[
            {
              icon: '🦀',
              title: 'Rust Powered',
              desc: 'Memory-safe, zero-cost abstractions, dan performa level C++ tanpa garbage collector.',
            },
            {
              icon: '⚡',
              title: 'Lightning Fast',
              desc: 'Hot Module Replacement Vite + kompilasi Rust inkremental. Development tanpa menunggu.',
            },
            {
              icon: '🔗',
              title: 'Monolith Modern',
              desc: 'Satu repo, satu deploy. Frontend & backend terintegrasi rapi lewat Inertia.js.',
            },
          ].map((feat) => (
            <div key={feat.title} style={{
              padding: '28px',
              borderRadius: '20px',
              border: `1px solid ${colors.cardBorder}`,
              background: colors.cardBg,
              transition: 'all 0.3s ease, border-color 0.3s, background-color 0.3s',
            }}>
              <div style={{ fontSize: '2rem', marginBottom: '12px' }}>{feat.icon}</div>
              <h4 style={{
                fontSize: '1.05rem',
                fontWeight: 800,
                color: colors.textMain,
                margin: '0 0 8px 0',
                transition: 'color 0.3s',
              }}>
                {feat.title}
              </h4>
              <p style={{
                fontSize: '0.85rem',
                color: colors.textMuted,
                lineHeight: 1.6,
                margin: 0,
                transition: 'color 0.3s',
              }}>
                {feat.desc}
              </p>
            </div>
          ))}
        </div>
      </main>
    </>
  );
}
