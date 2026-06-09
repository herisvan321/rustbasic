import React, { useEffect, useState } from 'react';
import { Link, Head } from '@inertiajs/react';
import { useTheme } from '../Layouts/AppLayout';
import { useRoute } from '../route';

interface AboutProps {
  title?: string;
  description?: string;
  backend?: string;
  frontend?: string;
  bridge?: string;
}

export default function About({ title, description, backend, frontend, bridge }: AboutProps) {
  const [mounted, setMounted] = useState(false);
  const { colors, isDark } = useTheme();
  const route = useRoute();

  useEffect(() => {
    setMounted(true);
  }, []);

  return (
    <>
      <Head>
        <title>Tentang RustBasic - Full-stack SPA Modern</title>
        <meta name="description" content="Pelajari arsitektur SPA modern berbasis Rust (Custom HTTP Engine) & React.js menggunakan Inertia.js." />
        <meta name="keywords" content="arsitektur rust, web framework, react spa, inertia js" />
        <meta property="og:title" content="Tentang RustBasic - Full-stack SPA Modern" />
        <meta property="og:description" content="Pelajari arsitektur SPA modern berbasis Rust & React." />
        <meta property="og:image" content="/logo.png" />
      </Head>

      {/* ===== MAIN CONTENT ===== */}
      <main style={{
        maxWidth: '1280px',
        margin: '0 auto',
        padding: '60px 24px 80px',
        flexGrow: 1,
        position: 'relative',
        zIndex: 10,
        width: '100%',
        boxSizing: 'border-box',
      }}>
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: '32px',
        }}>
          {/* Badge + Logo */}
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
                width: '56px',
                height: '56px',
                objectFit: 'contain',
                filter: colors.glowIntensity,
              }}
            />
            <div style={{
              display: 'inline-flex',
              padding: '8px 20px',
              borderRadius: '999px',
              fontSize: '0.8rem',
              fontWeight: 700,
              background: isDark ? 'rgba(232,82,14,0.08)' : 'rgba(232,82,14,0.05)',
              border: `1px solid ${isDark ? 'rgba(232,82,14,0.15)' : 'rgba(232,82,14,0.1)'}`,
              color: '#e8520e',
            }}>
              ⚙️ Framework Architecture Details
            </div>
          </div>

          {/* Heading */}
          <div style={{
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateY(0)' : 'translateY(20px)',
            transition: 'all 0.6s ease 0.1s',
          }}>
            <h1 style={{
              fontSize: 'clamp(2rem, 4vw, 3rem)',
              fontWeight: 900,
              letterSpacing: '-0.03em',
              lineHeight: 1.1,
              margin: '0 0 16px 0',
              color: colors.textMain,
              transition: 'color 0.3s',
            }}>
              {title}
            </h1>
            <p style={{
              fontSize: '1.1rem',
              color: colors.textMuted,
              lineHeight: 1.7,
              margin: 0,
              transition: 'color 0.3s',
            }}>
              {description}
            </p>
          </div>

          {/* Tech Spec Grid */}
          <div style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))',
            gap: '16px',
            marginTop: '8px',
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateY(0)' : 'translateY(20px)',
            transition: 'all 0.6s ease 0.2s',
          }}>
            {[
              { label: 'Backend Platform', value: backend, color: '#e8520e', desc: 'Super cepat, aman, & hemat memori.' },
              { label: 'Frontend Engine', value: frontend, color: '#ff8c42', desc: 'Kompilasi super cepat dengan HMR Vite.' },
              { label: 'Communication Bridge', value: bridge, color: isDark ? '#8b7355' : '#6b5438', desc: 'Kirim props tanpa API routing manual.' },
            ].map((spec) => (
              <div key={spec.label} style={{
                padding: '24px',
                borderRadius: '20px',
                border: `1px solid ${colors.cardBorder}`,
                background: colors.cardBg,
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
                transition: 'all 0.3s ease, border-color 0.3s, background-color 0.3s',
              }}>
                <span style={{
                  fontSize: '0.65rem',
                  fontWeight: 700,
                  color: spec.color,
                  letterSpacing: '0.1em',
                  textTransform: 'uppercase',
                }}>
                  {spec.label}
                </span>
                <span style={{ fontSize: '1.25rem', fontWeight: 800, color: colors.textMain, transition: 'color 0.3s' }}>
                  {spec.value}
                </span>
                <span style={{ fontSize: '0.75rem', color: colors.textMuted, transition: 'color 0.3s' }}>
                  {spec.desc}
                </span>
              </div>
            ))}
          </div>

          {/* Flow Diagram */}
          <div style={{
            padding: '32px',
            borderRadius: '24px',
            border: `1px solid ${colors.activeCardBorder}`,
            background: colors.activeCardBg,
            opacity: mounted ? 1 : 0,
            transform: mounted ? 'translateY(0)' : 'translateY(20px)',
            transition: 'all 0.6s ease 0.3s, background-color 0.3s, border-color 0.3s',
          }}>
            <h3 style={{
              fontSize: '1.15rem',
              fontWeight: 800,
              color: colors.textMain,
              margin: '0 0 20px 0',
              transition: 'color 0.3s',
            }}>
              ⚡ Bagaimana SPA Ini Bekerja?
            </h3>

            <div style={{
              display: 'flex',
              flexWrap: 'wrap',
              alignItems: 'center',
              justifyContent: 'space-between',
              gap: '16px',
            }}>
              {[
                { step: '1', title: 'User Click Link', desc: 'Inertia menangkap klik <Link> secara lokal tanpa reload browser.' },
                { step: '2', title: 'Axios X-Inertia', desc: 'Client mengirim request AJAX dengan header X-Inertia: true.' },
                { step: '3', title: 'Rust Inertia JSON', desc: 'Backend Rust langsung membalas dengan JSON berisi props & component name.' },
              ].map((item, i) => (
                <React.Fragment key={item.step}>
                  <div style={{
                    flex: '1 1 200px',
                    padding: '20px',
                    borderRadius: '16px',
                    border: `1px solid ${colors.cardBorder}`,
                    background: colors.cardBg,
                    textAlign: 'center',
                    transition: 'all 0.3s ease, border-color 0.3s, background-color 0.3s',
                  }}>
                    <div style={{
                      width: '32px',
                      height: '32px',
                      borderRadius: '50%',
                      background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                      display: 'inline-flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      fontWeight: 800,
                      fontSize: '0.85rem',
                      color: '#fff',
                      marginBottom: '12px',
                    }}>
                      {item.step}
                    </div>
                    <div style={{ fontWeight: 700, color: colors.textMain, fontSize: '0.9rem', marginBottom: '6px', transition: 'color 0.3s' }}>
                      {item.title}
                    </div>
                    <div style={{ fontSize: '0.8rem', color: colors.textMuted, lineHeight: 1.5, transition: 'color 0.3s' }}>
                      {item.desc}
                    </div>
                  </div>
                  {i < 2 && (
                    <div style={{
                      color: '#e8520e',
                      fontWeight: 800,
                      fontSize: '1.2rem',
                      flexShrink: 0,
                    }}>
                      →
                    </div>
                  )}
                </React.Fragment>
              ))}
            </div>
          </div>

          {/* Back Button */}
          <div style={{
            opacity: mounted ? 1 : 0,
            transition: 'all 0.6s ease 0.4s',
          }}>
            <Link
              href={route('home')}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                padding: '14px 28px',
                borderRadius: '14px',
                background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                fontWeight: 700,
                fontSize: '0.9rem',
                color: isDark ? '#ccc' : '#444',
                textDecoration: 'none',
                border: `1px solid ${colors.cardBorder}`,
                transition: 'all 0.3s ease, color 0.3s, background-color 0.3s',
              }}
            >
              ← Kembali ke Beranda
            </Link>
          </div>
        </div>
      </main>
    </>
  );
}
