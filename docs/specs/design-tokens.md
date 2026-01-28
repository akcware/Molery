# Design Tokens

## Overview

Single source of truth for all visual design values. Reference these tokens in components using CSS custom properties.

## Color Palette

### Brand Colors

| Token | Light Mode | Dark Mode | Usage |
|-------|------------|-----------|-------|
| `--color-accent-blue` | `#0066FF` | `#3B82F6` | Primary actions, links |
| `--color-accent-green` | `#22C55E` | `#22C55E` | Success states |
| `--color-accent-orange` | `#F97316` | `#F97316` | Warnings |
| `--color-accent-red` | `#EF4444` | `#EF4444` | Errors, destructive |

### Surface Colors

| Token | Light Mode | Dark Mode | Usage |
|-------|------------|-----------|-------|
| `--color-surface-primary` | `#FFFFFF` | `#1A1A1A` | Main background |
| `--color-surface-secondary` | `#F5F5F7` | `#2D2D2D` | Card backgrounds |
| `--color-surface-tertiary` | `#E5E5EA` | `#3D3D3D` | Subtle backgrounds |
| `--color-surface-elevated` | `#FFFFFF` | `#2D2D2D` | Modals, dropdowns |

### Content Colors

| Token | Light Mode | Dark Mode | Usage |
|-------|------------|-----------|-------|
| `--color-content-primary` | `#1D1D1F` | `#F5F5F7` | Primary text |
| `--color-content-secondary` | `#6E6E73` | `#A1A1A6` | Secondary text |
| `--color-content-tertiary` | `#8E8E93` | `#8E8E93` | Placeholder text |
| `--color-content-inverse` | `#FFFFFF` | `#1D1D1F` | Text on accent bg |

### Border Colors

| Token | Light Mode | Dark Mode | Usage |
|-------|------------|-----------|-------|
| `--color-border-primary` | `#D1D1D6` | `#3D3D3D` | Card borders |
| `--color-border-secondary` | `#E5E5EA` | `#2D2D2D` | Dividers |

---

## Typography

### Font Family

```css
--font-sans: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
--font-mono: 'SF Mono', SFMono-Regular, Menlo, Monaco, Consolas, monospace;
```

### Font Sizes

| Token | Size | Line Height | Usage |
|-------|------|-------------|-------|
| `--text-xs` | `12px` | `16px` | Labels, captions |
| `--text-sm` | `14px` | `20px` | Secondary content |
| `--text-base` | `16px` | `24px` | Body text |
| `--text-lg` | `18px` | `28px` | Subheadings |
| `--text-xl` | `20px` | `28px` | Section titles |
| `--text-2xl` | `24px` | `32px` | Page titles |
| `--text-3xl` | `30px` | `36px` | Hero numbers |

### Font Weights

| Token | Weight | Usage |
|-------|--------|-------|
| `--font-normal` | `400` | Body text |
| `--font-medium` | `500` | Emphasis |
| `--font-semibold` | `600` | Headings |
| `--font-bold` | `700` | Strong emphasis |

---

## Spacing

Based on 4px grid system.

| Token | Value | Usage |
|-------|-------|-------|
| `--space-0` | `0` | Reset |
| `--space-1` | `4px` | Tight spacing |
| `--space-2` | `8px` | Small gaps |
| `--space-3` | `12px` | Default gap |
| `--space-4` | `16px` | Component padding |
| `--space-5` | `20px` | Section gap |
| `--space-6` | `24px` | Card padding |
| `--space-8` | `32px` | Large sections |
| `--space-10` | `40px` | Page margins |
| `--space-12` | `48px` | Hero spacing |

---

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `--radius-sm` | `4px` | Small elements |
| `--radius-md` | `8px` | Buttons, inputs |
| `--radius-lg` | `12px` | Cards |
| `--radius-xl` | `16px` | Panels |
| `--radius-full` | `9999px` | Pills, avatars |

---

## Shadows

| Token | Value | Usage |
|-------|-------|-------|
| `--shadow-sm` | `0 1px 2px rgba(0,0,0,0.05)` | Subtle elevation |
| `--shadow-md` | `0 4px 6px rgba(0,0,0,0.1)` | Cards |
| `--shadow-lg` | `0 10px 15px rgba(0,0,0,0.1)` | Dropdowns |
| `--shadow-xl` | `0 20px 25px rgba(0,0,0,0.15)` | Modals |

---

## Transitions

| Token | Value | Usage |
|-------|-------|-------|
| `--transition-fast` | `150ms ease` | Hover states |
| `--transition-normal` | `200ms ease` | Standard transitions |
| `--transition-slow` | `300ms ease` | Expand/collapse |

---

## Z-Index Scale

| Token | Value | Usage |
|-------|-------|-------|
| `--z-base` | `0` | Default |
| `--z-dropdown` | `100` | Dropdowns |
| `--z-sticky` | `200` | Sticky headers |
| `--z-modal` | `300` | Modals |
| `--z-toast` | `400` | Toast notifications |

---

## Tailwind Configuration

**File:** `tailwind.config.js`

```javascript
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        surface: {
          primary: 'var(--color-surface-primary)',
          secondary: 'var(--color-surface-secondary)',
          tertiary: 'var(--color-surface-tertiary)',
          elevated: 'var(--color-surface-elevated)',
        },
        content: {
          primary: 'var(--color-content-primary)',
          secondary: 'var(--color-content-secondary)',
          tertiary: 'var(--color-content-tertiary)',
          inverse: 'var(--color-content-inverse)',
        },
        accent: {
          blue: 'var(--color-accent-blue)',
          green: 'var(--color-accent-green)',
          orange: 'var(--color-accent-orange)',
          red: 'var(--color-accent-red)',
        },
        border: {
          primary: 'var(--color-border-primary)',
          secondary: 'var(--color-border-secondary)',
        },
      },
      fontFamily: {
        sans: ['var(--font-sans)'],
        mono: ['var(--font-mono)'],
      },
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
      },
      borderRadius: {
        'xl': '12px',
        '2xl': '16px',
      },
    },
  },
  plugins: [],
};
```

---

## CSS Custom Properties

**File:** `src/app.css`

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  /* Fonts */
  --font-sans: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif;
  --font-mono: 'SF Mono', SFMono-Regular, Menlo, monospace;

  /* Light mode colors */
  --color-accent-blue: #0066FF;
  --color-accent-green: #22C55E;
  --color-accent-orange: #F97316;
  --color-accent-red: #EF4444;

  --color-surface-primary: #FFFFFF;
  --color-surface-secondary: #F5F5F7;
  --color-surface-tertiary: #E5E5EA;
  --color-surface-elevated: #FFFFFF;

  --color-content-primary: #1D1D1F;
  --color-content-secondary: #6E6E73;
  --color-content-tertiary: #8E8E93;
  --color-content-inverse: #FFFFFF;

  --color-border-primary: #D1D1D6;
  --color-border-secondary: #E5E5EA;
}

.dark {
  --color-accent-blue: #3B82F6;

  --color-surface-primary: #1A1A1A;
  --color-surface-secondary: #2D2D2D;
  --color-surface-tertiary: #3D3D3D;
  --color-surface-elevated: #2D2D2D;

  --color-content-primary: #F5F5F7;
  --color-content-secondary: #A1A1A6;
  --color-content-inverse: #1D1D1F;

  --color-border-primary: #3D3D3D;
  --color-border-secondary: #2D2D2D;
}
```
