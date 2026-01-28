# UI Screens Specification

## Layout Structure

```
┌──────────────────────────────────────────────────────────┐
│  Header (48px)                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Logo    App Title                    Settings  👤  │  │
│  └────────────────────────────────────────────────────┘  │
├────────────┬─────────────────────────────────────────────┤
│            │                                             │
│  Sidebar   │               Main Content                  │
│  (200px)   │                                             │
│            │                                             │
│  ┌──────┐  │  ┌─────────────────────────────────────┐   │
│  │ 🏠   │  │  │                                     │   │
│  │ Home │  │  │        Active Panel Content         │   │
│  │      │  │  │                                     │   │
│  │ 🧹   │  │  │                                     │   │
│  │ Clean│  │  │                                     │   │
│  │      │  │  │                                     │   │
│  │ 🗑️   │  │  └─────────────────────────────────────┘   │
│  │Uninst│  │                                             │
│  │      │  │                                             │
│  │ 📊   │  │                                             │
│  │Analyz│  │                                             │
│  │      │  │                                             │
│  │ ⚡   │  │                                             │
│  │Optim │  │                                             │
│  │      │  │                                             │
│  │ ℹ️   │  │                                             │
│  │Status│  │                                             │
│  └──────┘  │                                             │
│            │                                             │
└────────────┴─────────────────────────────────────────────┘
```

## Components

### Header

**File:** `src/lib/components/layout/Header.svelte`

| Element | Description |
|---------|-------------|
| Logo | App icon (32x32) |
| Title | "Molery" - `text-lg font-semibold` |
| Settings | Gear icon, opens settings modal |
| Profile | User avatar placeholder |

**Behavior:**
- Fixed position, stays visible during scroll
- Draggable region for window (macOS)

---

### Sidebar

**File:** `src/lib/components/layout/Sidebar.svelte`

| Property | Value |
|----------|-------|
| Width | `200px` (collapsed: `64px`) |
| Background | `surface-secondary` |
| Border | Right border `border-secondary` |

**Navigation Items:**

| Icon | Label | Panel ID | Keyboard |
|------|-------|----------|----------|
| 🏠 | Dashboard | `dashboard` | `⌘1` |
| 🧹 | Clean | `clean` | `⌘2` |
| 🗑️ | Uninstall | `uninstall` | `⌘3` |
| 📊 | Analyze | `analyze` | `⌘4` |
| ⚡ | Optimize | `optimize` | `⌘5` |
| ℹ️ | Status | `status` | `⌘6` |

**Active State:**
- Background: `surface-tertiary`
- Left border: `accent-blue` (3px)
- Icon/text: `accent-blue`

---

### Dashboard Panel

**File:** `src/lib/components/features/Dashboard.svelte`

```
┌─────────────────────────────────────────────────────────┐
│  Welcome to Molery                                      │
│  Your system optimization dashboard                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │ 📦 45GB  │  │ 🧹 2.3GB │  │ 📱 124   │  │ ✓ Good  │ │
│  │ Disk Used│  │ Cleanable│  │ Apps     │  │ Status  │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
│                                                         │
│  Quick Actions                                          │
│  ┌────────────────┐  ┌────────────────┐                │
│  │ 🧹 Quick Clean │  │ 📊 Full Scan   │                │
│  └────────────────┘  └────────────────┘                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**StatCard Component:**

| Prop | Type | Description |
|------|------|-------------|
| `icon` | `string` | Emoji or icon component |
| `value` | `string` | Main display value |
| `label` | `string` | Description text |
| `trend` | `'up' \| 'down' \| null` | Optional trend indicator |

---

### Clean Panel

**File:** `src/lib/components/features/CleanPanel.svelte`

```
┌─────────────────────────────────────────────────────────┐
│  System Cleanup                                         │
│  Scan and remove unnecessary files                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  [Scan Now]           Total: 2.3 GB cleanable          │
│                                                         │
│  Categories                                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │ ☑ Cache Files                           1.2 GB  │   │
│  │ ☑ System Logs                           450 MB  │   │
│  │ ☑ Trash                                 320 MB  │   │
│  │ ☐ Downloads                             180 MB  │   │
│  │ ☐ Xcode Derived Data                    150 MB  │   │
│  │ ☐ Homebrew Cache                         80 MB  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  [Select All]  [Deselect All]                          │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │           [🧹 Clean Selected (1.97 GB)]          │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**States:**

| State | UI Display |
|-------|------------|
| Initial | "Scan Now" button prominent |
| Scanning | Progress spinner, "Scanning..." text |
| Results | Category list with checkboxes |
| Cleaning | Progress bar, disable interactions |
| Complete | Success message, stats summary |

---

### Uninstall Panel

**File:** `src/lib/components/features/UninstallPanel.svelte`

```
┌─────────────────────────────────────────────────────────┐
│  App Uninstaller                                        │
│  Completely remove applications and their files         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  🔍 [Search apps...]                                   │
│                                                         │
│  Installed Applications (124)                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 📦 Visual Studio Code              245 MB  [🗑️] │   │
│  │ 📦 Docker Desktop                   1.8 GB  [🗑️] │   │
│  │ 📦 Slack                           380 MB  [🗑️] │   │
│  │ 📦 Spotify                         290 MB  [🗑️] │   │
│  │ ...                                              │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Uninstall Confirmation Modal:**

```
┌─────────────────────────────────────┐
│  Uninstall Visual Studio Code?      │
│                                     │
│  This will remove:                  │
│  • Application (245 MB)             │
│  • Preferences                      │
│  • Cache files                      │
│  • Support files                    │
│                                     │
│  [Cancel]        [Uninstall]        │
└─────────────────────────────────────┘
```

---

### Analyze Panel

**File:** `src/lib/components/features/AnalyzePanel.svelte`

```
┌─────────────────────────────────────────────────────────┐
│  Disk Analyzer                                          │
│  Visualize disk usage by folder                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Path: [/Users/username          ] [Browse] [Analyze]  │
│                                                         │
│  /Users/username (45.2 GB)                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │ ████████████████████████░░░░░░  Library  12 GB  │   │
│  │ ████████████████░░░░░░░░░░░░░░  Documents 8 GB  │   │
│  │ ██████████░░░░░░░░░░░░░░░░░░░░  Downloads 5 GB  │   │
│  │ ████████░░░░░░░░░░░░░░░░░░░░░░  Desktop   4 GB  │   │
│  │ ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░  Other    16 GB  │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  Click on a row to drill down                          │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

### Status Panel

**File:** `src/lib/components/features/StatusPanel.svelte`

```
┌─────────────────────────────────────────────────────────┐
│  System Status                                          │
│  Overview of your system health                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Disk Space                                            │
│  ┌─────────────────────────────────────────────────┐   │
│  │ ██████████████████████░░░░░░░░░░  45/120 GB     │   │
│  │                                   37.5% used    │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  mo CLI Status                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ ✓ Installed                      Version 0.2.1  │   │
│  │ Last cleanup: 2 days ago                        │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  Recommendations                                       │
│  • Clean 2.3 GB of cache files                        │
│  • Empty trash (320 MB)                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Toast Notifications

**File:** `src/lib/components/ui/Toast.svelte`

| Type | Icon | Border Color |
|------|------|--------------|
| Success | ✓ | `accent-green` |
| Error | ✕ | `accent-red` |
| Warning | ⚠ | `accent-orange` |
| Info | ℹ | `accent-blue` |

**Position:** Bottom-right, stacked vertically

**Animation:**
- Enter: Slide in from right
- Exit: Fade out

---

## Loading States

### Skeleton Loaders

```svelte
<div class="animate-pulse">
  <div class="h-4 bg-surface-tertiary rounded w-3/4"></div>
</div>
```

### Spinner

```svelte
<div class="animate-spin h-5 w-5 border-2 border-accent-blue border-t-transparent rounded-full"></div>
```

### Progress Bar

```svelte
<div class="h-2 bg-surface-tertiary rounded-full overflow-hidden">
  <div class="h-full bg-accent-blue transition-all" style="width: {progress}%"></div>
</div>
```
