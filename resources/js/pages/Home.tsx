import { Head, Link, usePage } from '@inertiajs/react';
import { useMemo, useState } from 'react';
import type { ChangeEvent, FormEvent, ReactNode } from 'react';

import type { SharedProps } from '../types';

const navLinks = [
  { label: 'Platform', href: '#platform' },
  { label: 'Solutions', href: '#solutions' },
  { label: 'Security', href: '#security' },
  { label: 'Resources', href: '#faq' },
  { label: 'Pricing', href: '#demo' },
];

const painPoints = [
  {
    index: '01',
    title: 'Fragmented data',
    body: 'Market feeds, filings, and internal models live in a dozen tools. Altitude brings them into one governed workspace.',
  },
  {
    index: '02',
    title: 'Manual analysis',
    body: 'Analysts spend days rebuilding the same models. Automate the repetitive work and keep humans on judgment.',
  },
  {
    index: '03',
    title: 'Slow decisions',
    body: 'By the time a memo is ready, the opportunity has moved. Get answers while the window is still open.',
  },
  {
    index: '04',
    title: 'Missed context',
    body: 'Institutional knowledge walks out the door. Capture it, connect it, and make it searchable for every deal team.',
  },
];

const platformFeatures = [
  {
    icon: 'layers',
    title: 'Unified data layer',
    body: 'Connect CRMs, data rooms, market feeds, and internal models in minutes.',
  },
  {
    icon: 'bolt',
    title: 'Intelligent automation',
    body: 'Route documents, extract terms, and draft first-pass analysis automatically.',
  },
  {
    icon: 'spark',
    title: 'Real-time insight',
    body: 'Ask questions in plain language and get sourced answers with citations.',
  },
  {
    icon: 'shield',
    title: 'Governed collaboration',
    body: 'Every output is versioned, permissioned, and ready for investment committee review.',
  },
];

const workflowFeatures = [
  {
    icon: 'search',
    title: 'Knowledge at Your Fingertips',
    body: 'Search across every document, email, and model your firm has ever produced — with citations you can trust.',
    mockup: 'search',
  },
  {
    icon: 'chart',
    title: 'Powerful Data Analysis',
    body: 'Blend market data with your own numbers and let Altitude surface the trends that matter.',
    mockup: 'chart',
  },
  {
    icon: 'clock',
    title: 'Timeline Analysis for Tasks',
    body: 'See every workstream, deadline, and dependency in one living view.',
    mockup: 'timeline',
  },
  {
    icon: 'doc',
    title: 'Reports Ready to Impress',
    body: 'Generate investment memos and LP updates in your house style, ready for review.',
    mockup: 'report',
  },
];

const securityFeatures = [
  {
    icon: 'shield',
    title: 'Compliance Overview',
    body: 'SOC 2 Type II, ISO 27001, and GDPR-aligned controls, audited annually.',
  },
  {
    icon: 'globe',
    title: 'External Security',
    body: 'Continuous penetration testing and a public bug bounty program.',
  },
  {
    icon: 'users',
    title: 'Trusted by Leaders',
    body: 'Deployed by regulated funds and Fortune 500 finance teams.',
  },
  {
    icon: 'lock',
    title: 'Platform Security',
    body: 'SSO, SCIM, granular roles, and encryption at rest and in transit.',
  },
  {
    icon: 'scale',
    title: 'Compliance & Governance',
    body: 'Configurable retention, audit logs, and data residency controls.',
  },
  {
    icon: 'cloud',
    title: 'Data Residency',
    body: 'Keep data in the region your regulators require, with no cross-border surprises.',
  },
];

const privateEquityFeatures = [
  {
    title: 'Deal Sourcing',
    body: 'Surface companies that match your thesis before they hit the market.',
  },
  {
    title: 'Due Diligence',
    body: 'Compress weeks of document review into a sourced, auditable summary.',
  },
  {
    title: 'Portfolio Monitoring',
    body: 'Track KPIs across the portfolio and flag risks as they emerge.',
  },
  {
    title: 'Value Creation',
    body: 'Turn operating data into a repeatable playbook for every portfolio company.',
  },
];

const faqs = [
  {
    question: "Is my firm's data used to train Altitude models?",
    answer:
      'No. Your data is never used to train shared models. Every workspace is isolated, and you can opt out of any model improvement program.',
  },
  {
    question: 'Is Altitude SOC 2 compliant?',
    answer:
      'Yes. Altitude is SOC 2 Type II certified and ISO 27001 aligned, with annual third-party audits.',
  },
  {
    question: 'What formats does Altitude support?',
    answer:
      'PDF, Word, Excel, PowerPoint, CSV, and most common data-room exports. Connectors cover the major CRMs and market-data providers.',
  },
  {
    question: 'What types of analysis can Altitude perform?',
    answer:
      'Comparable company analysis, precedent transactions, DCF support, document Q&A, portfolio KPI tracking, and custom workflows.',
  },
  {
    question: "How secure is Altitude's data?",
    answer:
      'Data is encrypted in transit and at rest, access is role-based, and every action is logged. Enterprise plans support customer-managed keys.',
  },
  {
    question: 'How long does it take to get started?',
    answer:
      'Most teams are live in under two weeks. A dedicated solutions engineer handles connectors, permissions, and onboarding.',
  },
  {
    question: 'What level of support does Altitude provide?',
    answer:
      'Every plan includes email and chat support. Enterprise plans add a named success manager and 24/7 incident response.',
  },
];

const footerColumns = [
  {
    title: 'Product',
    links: ['Platform', 'Integrations', 'Security', 'Pricing', 'Changelog'],
  },
  {
    title: 'Solutions',
    links: ['Private Equity', 'Venture Capital', 'Asset Management', 'Corporate Finance'],
  },
  {
    title: 'Resources',
    links: ['Documentation', 'API Reference', 'Guides', 'Blog', 'Status'],
  },
  {
    title: 'Company',
    links: ['About', 'Careers', 'Contact', 'Privacy', 'Terms'],
  },
];

const iconPaths: Record<string, ReactNode> = {
  spark: <path d="M12 3l1.9 5.1L19 10l-5.1 1.9L12 17l-1.9-5.1L5 10l5.1-1.9L12 3z" />,
  layers: (
    <>
      <path d="M12 3l9 5-9 5-9-5 9-5z" />
      <path d="M3 13l9 5 9-5" />
      <path d="M3 18l9 5 9-5" />
    </>
  ),
  bolt: <path d="M13 2L4 14h6l-1 8 9-12h-6l1-8z" />,
  shield: <path d="M12 3l7 3v6c0 4.5-3 7.5-7 9-4-1.5-7-4.5-7-9V6l7-3z" />,
  chart: (
    <>
      <path d="M4 20V10" />
      <path d="M10 20V4" />
      <path d="M16 20v-7" />
      <path d="M22 20H2" />
    </>
  ),
  search: (
    <>
      <circle cx="11" cy="11" r="8" />
      <path d="M21 21l-4.3-4.3" />
    </>
  ),
  clock: (
    <>
      <circle cx="12" cy="12" r="10" />
      <path d="M12 6v6l4 2" />
    </>
  ),
  doc: (
    <>
      <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z" />
      <path d="M14 2v6h6" />
      <path d="M8 13h8" />
      <path d="M8 17h5" />
    </>
  ),
  lock: (
    <>
      <rect x="5" y="11" width="14" height="10" rx="2" />
      <path d="M8 11V7a4 4 0 018 0v4" />
    </>
  ),
  globe: (
    <>
      <circle cx="12" cy="12" r="10" />
      <path d="M2 12h20" />
      <path d="M12 2a15 15 0 010 20 15 15 0 010-20z" />
    </>
  ),
  users: (
    <>
      <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2" />
      <circle cx="9" cy="7" r="4" />
      <path d="M23 21v-2a4 4 0 00-3-3.87" />
      <path d="M16 3.13a4 4 0 010 7.75" />
    </>
  ),
  scale: (
    <>
      <path d="M12 3v18" />
      <path d="M5 7l-3 6h6L5 7z" />
      <path d="M19 7l-3 6h6l-3-6z" />
      <path d="M5 7h14" />
    </>
  ),
  cloud: <path d="M18 10a4 4 0 00-7.5-2A5 5 0 006 18h12a4 4 0 000-8z" />,
  check: <path d="M20 6L9 17l-5-5" />,
  arrow: (
    <>
      <path d="M5 12h14" />
      <path d="M13 6l6 6-6 6" />
    </>
  ),
  plus: (
    <>
      <path d="M12 5v14" />
      <path d="M5 12h14" />
    </>
  ),
  minus: <path d="M5 12h14" />,
  menu: (
    <>
      <path d="M4 7h16" />
      <path d="M4 12h16" />
      <path d="M4 17h16" />
    </>
  ),
  close: (
    <>
      <path d="M6 6l12 12" />
      <path d="M18 6L6 18" />
    </>
  ),
};

function Icon({ name, className }: { name: string; className?: string }) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.6}
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
      aria-hidden="true"
    >
      {iconPaths[name]}
    </svg>
  );
}

function LogoMark({ className = 'h-7 w-7' }: { className?: string }) {
  return (
    <svg viewBox="0 0 32 32" className={className} aria-hidden="true">
      <defs>
        <linearGradient id="logoGrad" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stopColor="#8ab6ff" />
          <stop offset="100%" stopColor="#3b82f6" />
        </linearGradient>
      </defs>
      <path
        d="M16 3L29 27H3L16 3z"
        fill="none"
        stroke="url(#logoGrad)"
        strokeWidth="1.6"
        strokeLinejoin="round"
      />
      <path d="M16 11l6.5 12h-13L16 11z" fill="url(#logoGrad)" opacity="0.9" />
    </svg>
  );
}

function WaveField() {
  const { horizontal, vertical } = useMemo(() => {
    const W = 1600;
    const ROWS = 30;
    const COLS = 40;
    const ROW_GAP = 12;
    const TOP = 150;

    const surfaceY = (x: number, r: number) => {
      const baseY = TOP + r * ROW_GAP;
      const bump = Math.exp(-Math.pow((x - W / 2) / 430, 2)) * 110;
      const amp = 8 + (bump / 110) * 26;
      return (
        baseY -
        bump +
        Math.sin(x * 0.005 + r * 0.28) * amp +
        Math.sin(x * 0.011 + r * 0.5) * amp * 0.4
      );
    };

    const horizontal: string[] = [];
    for (let r = 0; r < ROWS; r++) {
      let d = '';
      for (let x = 0; x <= W; x += 16) {
        const y = surfaceY(x, r);
        d += `${x === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)} `;
      }
      horizontal.push(d);
    }

    const vertical: string[] = [];
    for (let c = 0; c <= COLS; c++) {
      const x = (c / COLS) * W;
      let d = '';
      for (let r = 0; r < ROWS; r++) {
        const y = surfaceY(x, r);
        d += `${r === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)} `;
      }
      vertical.push(d);
    }

    return { horizontal, vertical };
  }, []);

  return (
    <svg
      viewBox="0 0 1600 620"
      preserveAspectRatio="xMidYMid slice"
      className="h-full w-full"
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="waveStroke" x1="0" y1="0" x2="1" y2="0">
          <stop offset="0%" stopColor="#1e3a8a" stopOpacity="0.1" />
          <stop offset="30%" stopColor="#3b82f6" stopOpacity="0.55" />
          <stop offset="55%" stopColor="#93c5fd" stopOpacity="0.95" />
          <stop offset="75%" stopColor="#3b82f6" stopOpacity="0.6" />
          <stop offset="100%" stopColor="#1e3a8a" stopOpacity="0.1" />
        </linearGradient>
        <radialGradient id="waveGlow" cx="50%" cy="55%" r="50%">
          <stop offset="0%" stopColor="#3b82f6" stopOpacity="0.35" />
          <stop offset="60%" stopColor="#1d4ed8" stopOpacity="0.08" />
          <stop offset="100%" stopColor="#000000" stopOpacity="0" />
        </radialGradient>
        <filter id="waveBlur" x="-20%" y="-20%" width="140%" height="140%">
          <feGaussianBlur stdDeviation="6" />
        </filter>
      </defs>
      <rect width="1600" height="620" fill="url(#waveGlow)" />
      <g
        fill="none"
        stroke="url(#waveStroke)"
        strokeWidth="1"
        opacity="0.5"
        filter="url(#waveBlur)"
      >
        {horizontal.map((d, i) => (
          <path key={`h${i}`} d={d} />
        ))}
        {vertical.map((d, i) => (
          <path key={`v${i}`} d={d} />
        ))}
      </g>
      <g fill="none" stroke="url(#waveStroke)" strokeWidth="0.8" opacity="0.9">
        {horizontal.map((d, i) => (
          <path key={`hc${i}`} d={d} />
        ))}
        {vertical.map((d, i) => (
          <path key={`vc${i}`} d={d} />
        ))}
      </g>
    </svg>
  );
}

function CubeVisual() {
  const A = [200, 70];
  const B = [310, 130];
  const C = [200, 190];
  const D = [90, 130];
  const D2 = [90, 290];
  const C2 = [200, 350];
  const B2 = [310, 290];

  const lerp = (p: number[], q: number[], t: number) => [
    p[0] + (q[0] - p[0]) * t,
    p[1] + (q[1] - p[1]) * t,
  ];
  const line = (p: number[], q: number[]) => `M ${p[0]} ${p[1]} L ${q[0]} ${q[1]}`;

  const topGrid: string[] = [];
  const leftGrid: string[] = [];
  const rightGrid: string[] = [];
  for (let i = 1; i < 4; i++) {
    const t = i / 4;
    topGrid.push(line(lerp(D, A, t), lerp(C, B, t)));
    topGrid.push(line(lerp(A, B, t), lerp(D, C, t)));
    leftGrid.push(line(lerp(D, C, t), lerp(D2, C2, t)));
    leftGrid.push(line(lerp(D, D2, t), lerp(C, C2, t)));
    rightGrid.push(line(lerp(B, C, t), lerp(B2, C2, t)));
    rightGrid.push(line(lerp(B, B2, t), lerp(C, C2, t)));
  }

  return (
    <svg viewBox="0 0 400 420" className="h-auto w-full max-w-md" aria-hidden="true">
      <defs>
        <linearGradient id="cubeTop" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stopColor="#dbeafe" stopOpacity="0.95" />
          <stop offset="100%" stopColor="#93c5fd" stopOpacity="0.7" />
        </linearGradient>
        <linearGradient id="cubeLeft" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#3b82f6" stopOpacity="0.55" />
          <stop offset="100%" stopColor="#1e3a8a" stopOpacity="0.35" />
        </linearGradient>
        <linearGradient id="cubeRight" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#60a5fa" stopOpacity="0.7" />
          <stop offset="100%" stopColor="#1d4ed8" stopOpacity="0.45" />
        </linearGradient>
        <radialGradient id="cubeGlow" cx="50%" cy="50%" r="50%">
          <stop offset="0%" stopColor="#60a5fa" stopOpacity="0.55" />
          <stop offset="100%" stopColor="#60a5fa" stopOpacity="0" />
        </radialGradient>
        <filter id="cubeSoft" x="-50%" y="-50%" width="200%" height="200%">
          <feGaussianBlur stdDeviation="18" />
        </filter>
      </defs>
      <ellipse
        cx="200"
        cy="360"
        rx="150"
        ry="34"
        fill="url(#cubeGlow)"
        filter="url(#cubeSoft)"
      />
      <polygon
        points={`${A} ${B} ${C} ${D}`}
        fill="url(#cubeTop)"
        stroke="#bfdbfe"
        strokeWidth="1"
      />
      <polygon
        points={`${D} ${C} ${C2} ${D2}`}
        fill="url(#cubeLeft)"
        stroke="#93c5fd"
        strokeWidth="1"
      />
      <polygon
        points={`${B} ${C} ${C2} ${B2}`}
        fill="url(#cubeRight)"
        stroke="#bfdbfe"
        strokeWidth="1"
      />
      <g stroke="#e0f2fe" strokeWidth="0.6" opacity="0.5">
        {topGrid.map((d, i) => (
          <path key={`t${i}`} d={d} />
        ))}
        {leftGrid.map((d, i) => (
          <path key={`l${i}`} d={d} />
        ))}
        {rightGrid.map((d, i) => (
          <path key={`r${i}`} d={d} />
        ))}
      </g>
    </svg>
  );
}

function MountainVisual() {
  return (
    <svg
      viewBox="0 0 1440 800"
      preserveAspectRatio="xMidYMid slice"
      className="absolute inset-0 h-full w-full"
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="sky" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#0a1020" />
          <stop offset="55%" stopColor="#0d1a2e" />
          <stop offset="100%" stopColor="#05070d" />
        </linearGradient>
        <radialGradient id="peakGlow" cx="50%" cy="34%" r="42%">
          <stop offset="0%" stopColor="#ff7a3d" stopOpacity="0.55" />
          <stop offset="40%" stopColor="#c2410c" stopOpacity="0.2" />
          <stop offset="100%" stopColor="#000000" stopOpacity="0" />
        </radialGradient>
        <linearGradient id="backRange" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#16263f" />
          <stop offset="100%" stopColor="#0a1220" />
        </linearGradient>
        <linearGradient id="midRange" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#1b2f4a" />
          <stop offset="100%" stopColor="#0b1524" />
        </linearGradient>
        <linearGradient id="frontRange" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="#0f1d30" />
          <stop offset="100%" stopColor="#05070d" />
        </linearGradient>
      </defs>
      <rect width="1440" height="800" fill="url(#sky)" />
      <rect width="1440" height="800" fill="url(#peakGlow)" />
      {[
        [180, 120],
        [320, 80],
        [520, 150],
        [700, 60],
        [880, 110],
        [1040, 70],
        [1220, 140],
        [1340, 90],
      ].map(([cx, cy], i) => (
        <circle key={i} cx={cx} cy={cy} r={i % 3 === 0 ? 1.6 : 1} fill="#dbeafe" opacity="0.5" />
      ))}
      <polygon
        points="0,520 120,430 240,500 360,380 480,470 600,360 720,450 840,340 960,440 1080,370 1200,470 1320,400 1440,500 1440,800 0,800"
        fill="url(#backRange)"
      />
      <polygon
        points="0,600 160,500 300,570 440,460 560,540 700,430 820,520 960,440 1100,540 1240,470 1380,560 1440,520 1440,800 0,800"
        fill="url(#midRange)"
      />
      <polygon
        points="0,700 180,600 340,660 500,560 660,650 820,570 980,660 1140,590 1300,670 1440,610 1440,800 0,800"
        fill="url(#frontRange)"
      />
      <g fill="#cbd5e1" opacity="0.35">
        <polygon points="700,430 720,470 740,470 730,462 720,474 710,462" />
        <polygon points="440,460 458,492 476,492 468,485 458,496 450,485" />
        <polygon points="960,440 978,474 996,474 988,466 978,478 970,466" />
        <polygon points="1240,470 1256,500 1272,500 1264,493 1256,504 1248,493" />
      </g>
    </svg>
  );
}

function SearchMockup() {
  return (
    <div className="rounded-2xl border border-night/10 bg-mist p-4">
      <div className="flex items-center gap-2 rounded-xl border border-night/10 bg-white px-3 py-2.5 text-sm text-night/40">
        <Icon name="search" className="h-4 w-4" />
        <span>Search 12,480 documents…</span>
      </div>
      <div className="mt-3 space-y-2">
        {[
          'Q3 board deck — final.pdf',
          'Project Atlas — IC memo.docx',
          'Market map 2026.xlsx',
        ].map((item) => (
          <div
            key={item}
            className="flex items-center gap-2 rounded-lg bg-white px-3 py-2 text-xs text-night/60"
          >
            <Icon name="doc" className="h-3.5 w-3.5 text-accent" />
            <span className="truncate">{item}</span>
            <span className="ml-auto rounded-full bg-accent/10 px-2 py-0.5 text-[10px] font-medium text-accent">
              98%
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function ChartMockup() {
  const bars = [38, 52, 44, 68, 60, 82, 74, 92];
  return (
    <div className="rounded-2xl border border-night/10 bg-mist p-4">
      <div className="flex items-center justify-between text-xs text-night/50">
        <span>Revenue vs. plan</span>
        <span className="font-medium text-emerald-600">+18.4%</span>
      </div>
      <div className="mt-4 flex h-28 items-end gap-2">
        {bars.map((h, i) => (
          <div
            key={i}
            className="flex-1 rounded-t-md bg-gradient-to-t from-accent/30 to-accent"
            style={{ height: `${h}%` }}
          />
        ))}
      </div>
    </div>
  );
}

function TimelineMockup() {
  const steps = [
    { label: 'Sourcing', done: true },
    { label: 'Diligence', done: true },
    { label: 'IC review', done: false },
    { label: 'Close', done: false },
  ];
  return (
    <div className="rounded-2xl border border-night/10 bg-mist p-4">
      <div className="flex items-center justify-between">
        {steps.map((step, i) => (
          <div key={step.label} className="flex flex-1 flex-col items-center gap-2">
            <div className="flex w-full items-center">
              <div
                className={`h-0.5 flex-1 ${
                  i === 0 ? 'bg-transparent' : step.done ? 'bg-accent' : 'bg-night/10'
                }`}
              />
              <div
                className={`h-3 w-3 rounded-full ${
                  step.done ? 'bg-accent' : 'border-2 border-night/20 bg-white'
                }`}
              />
              <div
                className={`h-0.5 flex-1 ${
                  i === steps.length - 1
                    ? 'bg-transparent'
                    : steps[i + 1].done
                      ? 'bg-accent'
                      : 'bg-night/10'
                }`}
              />
            </div>
            <span className="text-[10px] font-medium text-night/50">{step.label}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

function ReportMockup() {
  return (
    <div className="rounded-2xl border border-night/10 bg-mist p-4">
      <div className="rounded-xl bg-white p-4 shadow-sm">
        <div className="flex items-center justify-between">
          <div className="h-2 w-24 rounded-full bg-night/20" />
          <span className="rounded-full bg-accent/10 px-2 py-0.5 text-[10px] font-medium text-accent">
            Draft
          </span>
        </div>
        <div className="mt-3 space-y-1.5">
          <div className="h-1.5 w-full rounded-full bg-night/10" />
          <div className="h-1.5 w-11/12 rounded-full bg-night/10" />
          <div className="h-1.5 w-4/5 rounded-full bg-night/10" />
        </div>
        <div className="mt-4 flex items-end gap-1.5">
          {[30, 45, 38, 60, 52, 72].map((h, i) => (
            <div
              key={i}
              className="flex-1 rounded-t bg-accent/40"
              style={{ height: `${h * 0.5}px` }}
            />
          ))}
        </div>
      </div>
    </div>
  );
}

function Mockup({ name }: { name: string }) {
  if (name === 'search') return <SearchMockup />;
  if (name === 'chart') return <ChartMockup />;
  if (name === 'timeline') return <TimelineMockup />;
  return <ReportMockup />;
}

function FaqItem({
  question,
  answer,
  open,
  onToggle,
}: {
  question: string;
  answer: string;
  open: boolean;
  onToggle: () => void;
}) {
  return (
    <div>
      <button
        type="button"
        onClick={onToggle}
        aria-expanded={open}
        className="flex w-full items-center justify-between gap-6 py-5 text-left"
      >
        <span className="text-base font-medium sm:text-lg">{question}</span>
        <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-night/15 text-night/60">
          <Icon name={open ? 'minus' : 'plus'} className="h-4 w-4" />
        </span>
      </button>
      <div
        className={`grid transition-all duration-300 ${
          open ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0'
        }`}
      >
        <div className="overflow-hidden">
          <p className="pb-5 pr-12 text-sm leading-relaxed text-night/55">{answer}</p>
        </div>
      </div>
    </div>
  );
}

function DemoForm() {
  const { props } = usePage<SharedProps>();
  const csrfToken = props.csrfToken ?? '';
  const [values, setValues] = useState({
    name: '',
    email: '',
    company: '',
    message: '',
  });
  const [status, setStatus] = useState<'idle' | 'sending' | 'sent' | 'error'>('idle');

  const update =
    (key: keyof typeof values) =>
    (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
      setValues((prev) => ({ ...prev, [key]: event.target.value }));
    };

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setStatus('sending');

    try {
      const message = values.company.trim()
        ? `Company: ${values.company.trim()}\n\n${values.message.trim()}`
        : values.message.trim();

      const response = await fetch('/contact', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-CSRF-TOKEN': csrfToken,
        },
        body: JSON.stringify({
          name: values.name.trim(),
          email: values.email.trim(),
          message,
        }),
      });

      if (!response.ok) throw new Error('Request failed');

      setStatus('sent');
      setValues({ name: '', email: '', company: '', message: '' });
    } catch {
      setStatus('error');
    }
  };

  const inputClass =
    'w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-3 text-sm text-white placeholder:text-white/30 outline-none transition focus:border-accent/60 focus:bg-white/[0.06] focus:ring-2 focus:ring-accent/20';

  return (
    <form onSubmit={submit} className="space-y-4">
      <div className="grid gap-4 sm:grid-cols-2">
        <div>
          <label htmlFor="demo-name" className="mb-1.5 block text-xs font-medium text-white/60">
            Name
          </label>
          <input
            id="demo-name"
            required
            minLength={3}
            value={values.name}
            onChange={update('name')}
            className={inputClass}
            placeholder="Jane Cooper"
          />
        </div>
        <div>
          <label htmlFor="demo-email" className="mb-1.5 block text-xs font-medium text-white/60">
            Work email
          </label>
          <input
            id="demo-email"
            type="email"
            required
            value={values.email}
            onChange={update('email')}
            className={inputClass}
            placeholder="jane@fund.com"
          />
        </div>
      </div>
      <div>
        <label htmlFor="demo-company" className="mb-1.5 block text-xs font-medium text-white/60">
          Company
        </label>
        <input
          id="demo-company"
          value={values.company}
          onChange={update('company')}
          className={inputClass}
          placeholder="Acme Capital"
        />
      </div>
      <div>
        <label htmlFor="demo-message" className="mb-1.5 block text-xs font-medium text-white/60">
          What would you like to solve?
        </label>
        <textarea
          id="demo-message"
          required
          minLength={10}
          rows={4}
          value={values.message}
          onChange={update('message')}
          className={inputClass}
          placeholder="Tell us about your team and workflows…"
        />
      </div>
      <button
        type="submit"
        disabled={status === 'sending'}
        className="inline-flex w-full items-center justify-center gap-2 rounded-xl bg-white px-5 py-3 text-sm font-semibold text-night transition hover:bg-white/90 disabled:cursor-not-allowed disabled:opacity-60"
      >
        {status === 'sending' ? 'Sending…' : 'Request a demo'}
        <Icon name="arrow" className="h-4 w-4" />
      </button>
      {status === 'sent' && (
        <p className="text-sm text-emerald-400">
          Thanks — we&apos;ll be in touch within one business day.
        </p>
      )}
      {status === 'error' && (
        <p className="text-sm text-red-400">
          Something went wrong. Please try again or email hello@altitude.ai.
        </p>
      )}
      <p className="text-center text-[11px] text-white/30">
        By submitting, you agree to our privacy policy.
      </p>
    </form>
  );
}

function Footer() {
  return (
    <footer className="border-t border-white/10">
      <div className="mx-auto max-w-7xl px-6 py-14 lg:px-10">
        <div className="grid gap-10 lg:grid-cols-[1.4fr_repeat(4,1fr)]">
          <div>
            <Link href="/" className="flex items-center gap-2.5">
              <LogoMark />
              <span className="text-sm font-semibold tracking-[0.28em]">ALTITUDE</span>
            </Link>
            <p className="mt-4 max-w-xs text-sm leading-relaxed text-white/40">
              AI-native intelligence for finance teams. Make complex decisions simple.
            </p>
            <div className="mt-6 flex gap-2">
              {['LinkedIn', 'X', 'GitHub'].map((social) => (
                <a
                  key={social}
                  href="#"
                  className="rounded-full border border-white/10 px-3 py-1.5 text-xs text-white/50 transition hover:border-white/25 hover:text-white"
                >
                  {social}
                </a>
              ))}
            </div>
          </div>
          {footerColumns.map((column) => (
            <div key={column.title}>
              <h3 className="text-xs font-semibold uppercase tracking-wider text-white/40">
                {column.title}
              </h3>
              <ul className="mt-4 space-y-2.5">
                {column.links.map((link) => (
                  <li key={link}>
                    <a
                      href="#"
                      className="text-sm text-white/60 transition hover:text-white"
                    >
                      {link}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
        <div className="mt-12 flex flex-col items-start justify-between gap-4 border-t border-white/10 pt-6 text-xs text-white/35 sm:flex-row sm:items-center">
          <p>© {new Date().getFullYear()} Altitude Intelligence, Inc. All rights reserved.</p>
          <div className="flex gap-5">
            <a href="#" className="transition hover:text-white/70">
              Privacy
            </a>
            <a href="#" className="transition hover:text-white/70">
              Terms
            </a>
            <a href="#" className="transition hover:text-white/70">
              Security
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}

export default function Home() {
  const [menuOpen, setMenuOpen] = useState(false);
  const [openFaq, setOpenFaq] = useState<number | null>(0);

  return (
    <div className="min-h-screen bg-night font-sans text-white antialiased">
      <Head title="AI for Finance" />

      <header className="absolute inset-x-0 top-0 z-50">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-5 lg:px-10">
          <Link href="/" className="flex items-center gap-2.5">
            <LogoMark />
            <span className="text-sm font-semibold tracking-[0.28em] text-white">ALTITUDE</span>
          </Link>

          <nav className="hidden items-center gap-8 text-sm text-white/70 lg:flex">
            {navLinks.map((link) => (
              <a key={link.label} href={link.href} className="transition hover:text-white">
                {link.label}
              </a>
            ))}
          </nav>

          <div className="hidden items-center gap-5 lg:flex">
            <Link href="/login" className="text-sm text-white/70 transition hover:text-white">
              Log in
            </Link>
            <Link
              href="/register"
              className="rounded-full bg-white px-5 py-2.5 text-sm font-semibold text-night transition hover:bg-white/90"
            >
              Get started
            </Link>
          </div>

          <button
            type="button"
            onClick={() => setMenuOpen((open) => !open)}
            aria-label="Toggle navigation"
            aria-expanded={menuOpen}
            className="flex h-10 w-10 items-center justify-center rounded-full border border-white/15 text-white lg:hidden"
          >
            <Icon name={menuOpen ? 'close' : 'menu'} className="h-5 w-5" />
          </button>
        </div>

        {menuOpen && (
          <div className="mx-6 rounded-2xl border border-white/10 bg-navy/95 p-5 backdrop-blur lg:hidden">
            <nav className="flex flex-col gap-4 text-sm text-white/70">
              {navLinks.map((link) => (
                <a
                  key={link.label}
                  href={link.href}
                  onClick={() => setMenuOpen(false)}
                  className="transition hover:text-white"
                >
                  {link.label}
                </a>
              ))}
            </nav>
            <div className="mt-5 flex flex-col gap-3 border-t border-white/10 pt-5">
              <Link href="/login" className="text-sm text-white/70">
                Log in
              </Link>
              <Link
                href="/register"
                className="rounded-full bg-white px-5 py-2.5 text-center text-sm font-semibold text-night"
              >
                Get started
              </Link>
            </div>
          </div>
        )}
      </header>

      <section className="relative min-h-[92vh] overflow-hidden bg-night">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(37,99,235,0.18),transparent_55%)]" />
        <div className="pointer-events-none absolute inset-x-0 bottom-0 h-[68%]">
          <WaveField />
        </div>

        <div className="relative z-10 mx-auto flex min-h-[92vh] max-w-7xl flex-col px-6 pt-32 pb-14 lg:px-10">
          <div className="max-w-3xl">
            <p className="text-xs font-medium uppercase tracking-[0.24em] text-accent-2/80">
              AI-native intelligence for finance teams
            </p>
            <h1 className="mt-6 text-5xl font-semibold leading-[1.02] tracking-tight sm:text-6xl lg:text-7xl">
              AI for Finance
              <br />
              <span className="text-white/90">Complex Decisions</span>
              <br />
              <span className="bg-gradient-to-r from-accent-2 via-accent to-accent-2 bg-clip-text text-transparent">
                made simple
              </span>
            </h1>
            <p className="mt-6 max-w-xl text-base leading-relaxed text-white/55">
              Altitude turns fragmented market data, documents, and models into clear,
              defensible decisions — in seconds, not weeks.
            </p>
          </div>

          <div className="mt-auto pt-16">
            <div className="flex flex-wrap gap-3">
              <a
                href="#demo"
                className="inline-flex items-center gap-2 rounded-full bg-white px-6 py-3 text-sm font-semibold text-night transition hover:bg-white/90"
              >
                Request a demo
                <Icon name="arrow" className="h-4 w-4" />
              </a>
              <a
                href="#platform"
                className="inline-flex items-center gap-2 rounded-full border border-white/20 px-6 py-3 text-sm font-semibold text-white transition hover:border-white/40 hover:bg-white/5"
              >
                Explore the platform
              </a>
            </div>
            <p className="mt-6 max-w-md text-xs leading-relaxed text-white/35">
              Trusted by 200+ investment teams across private equity, venture, and asset
              management.
            </p>
          </div>
        </div>
      </section>

      <section className="bg-mist text-night">
        <div className="mx-auto max-w-7xl px-6 py-24 lg:px-10 lg:py-32">
          <div className="grid gap-10 lg:grid-cols-2 lg:items-end">
            <h2 className="text-4xl font-semibold leading-[1.05] tracking-tight sm:text-5xl lg:text-6xl">
              Too much data
              <br />
              <span className="text-night/35">Not Enough time</span>
            </h2>
            <p className="max-w-md text-base leading-relaxed text-night/60 lg:justify-self-end">
              Finance teams are drowning in information and starved for insight. Altitude
              closes the gap between the data you have and the decisions you need to make.
            </p>
          </div>

          <div className="mt-16 grid gap-px overflow-hidden rounded-2xl border border-night/10 bg-night/10 sm:grid-cols-2 lg:grid-cols-4">
            {painPoints.map((point) => (
              <div key={point.index} className="bg-mist p-6 lg:p-8">
                <span className="text-xs font-medium text-night/40">{point.index}</span>
                <h3 className="mt-6 text-lg font-semibold">{point.title}</h3>
                <p className="mt-2 text-sm leading-relaxed text-night/55">{point.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="platform" className="relative overflow-hidden bg-night text-white">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_center,rgba(37,99,235,0.12),transparent_60%)]" />
        <div className="relative mx-auto grid max-w-7xl gap-12 px-6 py-24 lg:grid-cols-2 lg:items-center lg:px-10 lg:py-32">
          <div className="flex items-center justify-center">
            <CubeVisual />
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/[0.03] p-8 backdrop-blur lg:p-10">
            <p className="text-xs font-medium uppercase tracking-[0.24em] text-accent-2/80">
              One platform
            </p>
            <h2 className="mt-4 text-4xl font-semibold tracking-tight sm:text-5xl">
              Your Workflows
              <br />
              <span className="text-accent-2">Unified and Enhanced</span>
            </h2>
            <p className="mt-4 text-sm leading-relaxed text-white/55">
              Replace the patchwork of tools with a single intelligence layer built for
              investment teams.
            </p>

            <ul className="mt-8 space-y-5">
              {platformFeatures.map((feature) => (
                <li key={feature.title} className="flex gap-4">
                  <span className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-white/10 bg-white/5 text-accent-2">
                    <Icon name={feature.icon} className="h-5 w-5" />
                  </span>
                  <div>
                    <h3 className="text-sm font-semibold">{feature.title}</h3>
                    <p className="mt-1 text-sm leading-relaxed text-white/50">{feature.body}</p>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </section>

      <section className="bg-mist text-night">
        <div className="mx-auto grid max-w-7xl gap-16 px-6 py-24 lg:grid-cols-[0.8fr_1.2fr] lg:px-10 lg:py-32">
          <div className="lg:sticky lg:top-24 lg:self-start">
            <p className="text-xs font-medium uppercase tracking-[0.24em] text-accent">
              Built around you
            </p>
            <h2 className="mt-4 text-4xl font-semibold leading-[1.05] tracking-tight sm:text-5xl">
              Designed to Your <span className="text-accent">Workflows</span>
            </h2>
            <p className="mt-5 max-w-sm text-base leading-relaxed text-night/60">
              Altitude adapts to how your team already works — not the other way around.
              Every surface is built for speed, clarity, and trust.
            </p>
            <a
              href="#demo"
              className="mt-8 inline-flex items-center gap-2 rounded-full bg-night px-6 py-3 text-sm font-semibold text-white transition hover:bg-night/90"
            >
              Explore features
              <Icon name="arrow" className="h-4 w-4" />
            </a>
          </div>

          <div className="space-y-6">
            {workflowFeatures.map((feature) => (
              <div
                key={feature.title}
                className="rounded-3xl border border-night/10 bg-white p-6 shadow-sm lg:p-8"
              >
                <div className="flex items-start gap-4">
                  <span className="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-accent/10 text-accent">
                    <Icon name={feature.icon} className="h-5 w-5" />
                  </span>
                  <div>
                    <h3 className="text-xl font-semibold">{feature.title}</h3>
                    <p className="mt-2 text-sm leading-relaxed text-night/55">{feature.body}</p>
                  </div>
                </div>
                <div className="mt-6">
                  <Mockup name={feature.mockup} />
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="security" className="bg-mist-2 text-night">
        <div className="mx-auto max-w-7xl px-6 py-24 lg:px-10 lg:py-32">
          <div className="grid gap-10 lg:grid-cols-2 lg:items-end">
            <h2 className="text-4xl font-semibold leading-[1.05] tracking-tight sm:text-5xl lg:text-6xl">
              Security you
              <br />
              <span className="text-accent">can Trust</span>
            </h2>
            <p className="max-w-md text-base leading-relaxed text-night/60 lg:justify-self-end">
              Your data is your edge. Altitude protects it with the controls, certifications,
              and transparency that regulated finance teams demand.
            </p>
          </div>

          <div className="mt-14 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {securityFeatures.map((feature) => (
              <div
                key={feature.title}
                className="rounded-2xl border border-night/10 bg-white p-6 transition hover:border-accent/30 hover:shadow-sm"
              >
                <span className="flex h-10 w-10 items-center justify-center rounded-xl bg-accent/10 text-accent">
                  <Icon name={feature.icon} className="h-5 w-5" />
                </span>
                <h3 className="mt-5 text-base font-semibold">{feature.title}</h3>
                <p className="mt-2 text-sm leading-relaxed text-night/55">{feature.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="solutions" className="relative overflow-hidden bg-night text-white">
        <MountainVisual />
        <div className="relative z-10 mx-auto max-w-7xl px-6 py-24 lg:px-10 lg:py-32">
          <div className="max-w-2xl">
            <p className="text-xs font-medium uppercase tracking-[0.24em] text-accent-2/80">
              Industry solutions
            </p>
            <h2 className="mt-4 text-5xl font-semibold tracking-tight sm:text-6xl">
              Private Equity
            </h2>
            <p className="mt-5 max-w-xl text-base leading-relaxed text-white/60">
              From first look to final close, Altitude gives deal teams a single source of
              truth — and the speed to act on it.
            </p>
          </div>

          <div className="mt-20 grid gap-8 border-t border-white/10 pt-10 sm:grid-cols-2 lg:grid-cols-4">
            {privateEquityFeatures.map((feature) => (
              <div key={feature.title}>
                <h3 className="text-sm font-semibold text-white">{feature.title}</h3>
                <p className="mt-2 text-sm leading-relaxed text-white/50">{feature.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="faq" className="bg-mist text-night">
        <div className="mx-auto max-w-4xl px-6 py-24 lg:py-32">
          <h2 className="text-4xl font-semibold leading-[1.05] tracking-tight sm:text-5xl">
            Got Questions?
            <br />
            <span className="text-accent">We&apos;ve got answers</span>
          </h2>

          <div className="mt-12 divide-y divide-night/10 border-y border-night/10">
            {faqs.map((faq, index) => (
              <FaqItem
                key={faq.question}
                question={faq.question}
                answer={faq.answer}
                open={openFaq === index}
                onToggle={() => setOpenFaq(openFaq === index ? null : index)}
              />
            ))}
          </div>
        </div>
      </section>

      <section id="demo" className="relative overflow-hidden bg-night text-white">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(37,99,235,0.16),transparent_55%)]" />
        <div className="relative mx-auto grid max-w-7xl gap-16 px-6 py-24 lg:grid-cols-2 lg:px-10 lg:py-32">
          <div>
            <h2 className="text-4xl font-semibold leading-[1.05] tracking-tight sm:text-5xl">
              Ready to transform
              <br />
              your workflow?
            </h2>
            <p className="mt-5 max-w-md text-base leading-relaxed text-white/55">
              See how Altitude can turn your firm&apos;s data into a durable advantage. Book a
              personalized walkthrough with our team.
            </p>

            <div className="mt-10 space-y-4 text-sm text-white/50">
              <p className="flex items-center gap-3">
                <span className="flex h-8 w-8 items-center justify-center rounded-full border border-white/10 text-accent-2">
                  <Icon name="spark" className="h-4 w-4" />
                </span>
                hello@altitude.ai
              </p>
              <p className="flex items-center gap-3">
                <span className="flex h-8 w-8 items-center justify-center rounded-full border border-white/10 text-accent-2">
                  <Icon name="globe" className="h-4 w-4" />
                </span>
                San Francisco · New York · London
              </p>
              <p className="flex items-center gap-3">
                <span className="flex h-8 w-8 items-center justify-center rounded-full border border-white/10 text-accent-2">
                  <Icon name="clock" className="h-4 w-4" />
                </span>
                Response within one business day
              </p>
            </div>
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/[0.03] p-6 backdrop-blur lg:p-8">
            <DemoForm />
          </div>
        </div>

        <Footer />
      </section>
    </div>
  );
}
