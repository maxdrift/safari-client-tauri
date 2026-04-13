# Safari Client — Product Requirements Document (PRD)

**Audience:** Anyone (including an LLM) designing or reimplementing this product. This document describes **what the product must do** and **what constraints apply**. It does **not** mandate a single framework or mimic the reference codebase layout, but it **does** include **implementation and distribution constraints** (§14)—including cross-platform desktop, efficient imaging, and standalone distribution—that any stack must satisfy.

---

## 1. Product overview

### 1.1 Problem

**Competition participants** in **Safari Fotosub**–style underwater photo competitions copy the photos they took during the event onto their computer and need a **desktop tool** to:

- Review many **local image files** in one place.
- Mark each photo with a **category** (three options: excluded, fixed points, or jury).
- Assign a **species** from an **official list** (with scoring metadata).
- Optionally apply **simple orientation adjustments** (rotate, mirror, flip) without a separate photo editor.
- **Reorder** photos when needed.
- Produce a **single CSV file** (“scheda concorrente”) that downstream processes can consume.

### 1.2 Solution (one sentence)

A **single-window** application that manages a **working set of photos**, supports **classification and labeling**, and **exports validated competitor data** as a semicolon-separated CSV.

### 1.3 Language

All user-visible text (labels, buttons, dialogs, errors) is **Italian**.

---

## 2. Users and goals

### 2.1 Primary user

A **participant** (photographer entering the competition) who is not assumed to be technical: they use file dialogs, a grid of thumbnails, and clear validation messages. They work **offline** with files on disk; the app does not host images on the web.

### 2.2 User goals

| Goal | Success looks like |
|------|---------------------|
| Load many photos quickly | User picks multiple images; they appear in a grid with previews. |
| Classify each photo | User can set category and species without losing work accidentally. |
| Work in batches | User can select several photos and apply the same category or species. |
| Export safely | User gets a CSV only when **validation passes**; clear errors otherwise. |
| Resume later | **Photo list, order, categories, species assignments, and image adjustments** survive closing the app (see §9). |

---

## 3. Core concepts (domain language)

### 3.1 Slide

A **slide** is one entry in the working set. It corresponds to **one image file** the user added.

- **Identity:** The slide is identified by its **file name** (not the full path). Two different paths that share the same file name are treated as the **same slide**; duplicates collapse so **one entry remains** (last wins if the same name is added again).
- **Stored reference:** The app keeps the path (or equivalent) to the **original image file** on disk for display and export.
- **Preview:** The app generates **smaller cached copies** of each image at fixed widths for performance; these are derived data and can be recreated from the original.

### 3.2 Judging category (three states)

Each slide is in exactly **one** of three categories:

| Category | Meaning (product sense) |
|----------|-------------------------|
| **Excluded** | Out of the running for the next export step (Italian: *Scartata* / *Scartate*). |
| **Fixed points** | Counts for fixed scoring (Italian: *Punt. fisso*). |
| **Jury** | In play for jury (Italian: *In giuria*). |

**Interaction model:**

- **Single slide:** The user can **cycle** the category in fixed order: Excluded → Fixed → Jury → Excluded → …
- **Multiple selected slides:** The user can **set** all selected slides to a **chosen** category in one action (no cycling).

### 3.3 Species (subject)

Each slide may have an assigned **species** from a **bundled catalog** (see §6). Assignments use a **numeric id** from that catalog.

- **No species / clear:** Stored on the slide as **`subjectid` 0**. That value is **not** read from the CSV file; it is a **sentinel** meaning “unassigned.” The species picker can still offer a **“Rimuovi specie”** choice as **one row in the same list** as real species so clearing does not need a separate control (see §6.3).
- Export rules require non-zero species for rows that are exported (see §8).
- **At most one use per species among non-excluded slides:** Across all slides whose category is **Fixed** or **Jury**, each catalog **`subj_id`** may appear **at most once**. A participant must not assign the same species to one slide as **Jury** and another as **Fixed**—that would duplicate `subj_id` in the export (§8.4). Slides in **Excluded** may still carry any assignment for convenience, but excluded rows are not exported.

### 3.4 Selection

Slides can be **selected** or not, independent of category. Selection is used for **bulk actions** (set category, set species, delete from working set).

### 3.5 Image adjustments (orientation)

Each slide may have **non-destructive** metadata for **rotation** (e.g. 90° steps) and **mirror/flip** (horizontal and/or vertical), applied when building previews and when the user views the full image. **Original files on disk are not overwritten** by default; the app stores enough data to reproduce the same appearance after restart (see §9).

**Composition and canonical form (for export):**

- User actions (**rotate 90° CW**, **horizontal flip**, **vertical flip**) are **composed in order** onto the slide’s current **orientation state**. Algebraically these maps live in the **dihedral group D₄** (the **eight** orthogonal symmetries of a square pixel grid): every reachable state is **equivalent** to one of those eight.
- The app **does not** export the raw list of taps. It **maintains** a single **canonical** transform per slide, updating it after each operation so that **equivalent sequences collapse** (e.g. four 90° rotations → identity; combinations that cancel map to **`transform_id` 0** per §8.6).
- **Persistence** stores this canonical state (or an equivalent representation); **export** writes it as one **integer column** **`transform_id`** in **`0`–`7`** (see §8.3, §8.6). This is an **efficient** encoding (three bits of information; CSV stores a decimal digit).

---

## 4. Main user journeys

### 4.1 First load — empty state

1. User opens the app.
2. User sees a clear message that they should load images to begin, and a **prominent control** to load images.
3. User invokes the control; the system **file picker** allows **multiple** files with extensions typical for photos (e.g. jpg, png, gif).

### 4.2 Load images

1. User selects one or more image files.
2. For each file, the app adds a slide (or reconciles duplicate file names as in §3.1).
3. The app shows a **grid of thumbnails** with responsive columns (roughly **1 to 4 columns** depending on available width).
4. While **no slide is selected**, a **floating action** to add more images remains available.

### 4.3 Review one photo

1. User sees filename and visual state (color or badge) for category.
2. User may **click the image** to open a **full-screen or lightbox** view for closer inspection; **orientation controls** (rotate, mirror/flip) are available where appropriate so participants can adjust framing without leaving the app (see §3.5, §5.7).
3. User may **toggle selection** via a dedicated control on the tile.
4. On hover (when not selected), the user can see **overlay actions**: filename, control to **cycle category**, and control to **open the species picker** for that slide.

### 4.4 Filter by category

1. Below the top bar, **tabs** show counts: **All**, **Excluded**, **Fixed**, **Jury**.
2. Choosing a tab **filters** which slides appear in the grid. The **underlying order** of the full list is unchanged; only the view changes.

### 4.5 Reorder slides

1. Only when the **All** tab is active, the user may **drag and drop** slides to **reorder** the master list.
2. When any **filtered** tab is active, **reordering is disabled** (the product treats reorder as an operation on the full list).

### 4.6 Bulk actions

When **at least one** slide is selected, the **top bar switches** to a **selection mode**:

- Show how many slides are selected (Italian copy distinguishes singular vs plural).
- **Deselect all**, **select all** (respecting the **current filter**: only slides visible in that filter are selected/deselected), **set category** via a menu (“Giudica”), **assign species** for the selection, **delete** selected slides from the working set (with confirmation), **remove selection**.

**After** applying category or species to the selection, **selection clears** for those slides (product behavior: no accidental double-apply).

### 4.7 Species picker (modal)

1. User opens the picker for one slide or many selected slides.
2. A **search field** filters the catalog by **Italian common name** or **scientific name** (case-insensitive).
3. User picks a row; **confirm** assigns that species id to **all** slide ids targeted by the action.
4. To clear species, the user picks **“Rimuovi specie”**, which sets **`subjectid` to 0** (in the reference app this is implemented as an extra row in the same searchable list—not a row in the official catalog file).

### 4.8 Delete slides from the working set

1. User confirms deletion.
2. The app removes those slides from the **working set** and deletes the **cached preview files** associated with those slides (if any).

### 4.9 Export CSV

1. When there is at least one slide in the working set, the user can choose **Export CSV** from the default bar.
2. User chooses a save location; suggested default file name: **`scheda_concorrente.csv`**.
3. The app runs **validation** (§8). If validation fails, show an **error dialog** and **do not** write a file.
4. If validation succeeds, write the file and show a **success** message.

---

## 5. Functional requirements

### 5.1 Platform and environment

- **FR1:** The product is a **desktop application** (not a web page in a browser tab as the primary delivery).
- **FR2:** The product works with **local files** chosen by the user; it does not require uploading images to a server for core workflows.

### 5.2 Images

- **FR3:** Support common raster formats as reflected in the file picker (at minimum **JPEG, PNG, GIF**).
- **FR4:** Generate **three** cached preview widths for each original: **350px**, **512px**, and **1024px** wide (scale **inside** those bounds, preserve aspect ratio; implementation may use any library). Previews must reflect each slide’s **orientation metadata** (§3.5).
- **FR5:** On startup, if an original file still exists but **cached previews are missing**, **regenerate** previews for that slide automatically.

### 5.3 Slides list behavior

- **FR6:** Slide identity is the **file name**; duplicate names collapse (§3.1).
- **FR7:** Persist the **ordered list** of slides and each slide’s **category**, **species id**, and **orientation metadata** (§3.5) across sessions (§9).
- **FR8:** Do **not** persist which **filter tab** was active or whether a **modal** was open (fresh session starts with sensible defaults).

### 5.4 Category and selection

- **FR9:** Implement **single-slide cycle** and **bulk set** as in §3.2 and §4.6.
- **FR10:** **Select all / deselect all** applies only to slides that match the **current filter** (or the logical equivalent: all states when viewing “All”).

### 5.5 Export

- **FR11:** Export format and validation **must** match §8.

### 5.6 Optional product behavior (present in reference)

- **FR12 (optional):** Usage analytics (e.g. screen views, events) may be included; **anonymous user id** may persist locally. A rewrite may omit or replace this if not required.

### 5.7 Image transforms

- **FR13:** Provide **rotation** (90° steps), **horizontal flip** (mirror), and **vertical flip** as fast operations on the slide’s **working representation** for UI and export preview; **do not overwrite** original image files by default (§3.5). Each operation **updates the slide’s canonical orientation** by composition in **D₄** so redundant sequences **collapse** to one of eight states (§3.5).
- **FR14:** After orientation changes, update or **regenerate** cached previews (§7) so thumbnails and lightbox stay consistent.
- **FR16:** On **CSV export**, write each exported row’s **`transform_id`** (§8.3, §8.6). It must match the slide’s **canonical** orientation at export time (**`0`** means identity / no net transform relative to the original file as loaded).

### 5.8 Species usage overview (full list)

- **FR15:** Provide a **full list of all catalog species** (scrollable), searchable by **Italian common name** or **scientific name** (case-insensitive), using the **same search behavior** as the species picker (§4.7). Each row is **highlighted** according to current slide assignments: **green** if at least one **Jury** slide uses that species; **yellow** if at least one **Fixed** slide uses that species; **no highlight** if no **Jury** or **Fixed** slide uses that species (species appearing only on **Excluded** slides or unused appears unhighlighted). Under the **at most one** rule for `subj_id` among **Jury ∪ Fixed** (§3.3), a valid working set never assigns both Jury and Fixed to the same species—so a row is not both green and yellow at once.

---

## 6. Species catalog (bundled content)

### 6.1 Source

The product ships a **catalog file** (in the reference app: `elenco_pesci_2019.csv`). A reimplementation must ship **equivalent data**—either the same file or embedded data.

### 6.2 Row format

Each non-empty line:

**`id; commonName; scientificName; coefficient; version`**

- `id`: positive integer, unique in the file.
- `commonName`: Italian label shown in lists.
- `scientificName`: Latin name, shown as secondary text.
- `coefficient`: numeric factor used in export.
- `version`: text token (e.g. `v0`), used in export.

### 6.3 “Rimuovi specie” in the picker (not catalog data)

The official CSV (§6.2) only contains **real** species with **positive** ids. The reference app **injects** a synthetic row when building the list for the species modal: **`id = 0`**, label **“Rimuovi specie”**. That is a **UI convenience** so the same list component can offer “clear assignment” without a separate button. Choosing it sets the slide’s **`subjectid` to 0** (unassigned). It is **not** part of the shipped competition catalog.

---

## 7. Preview file lifecycle (product rules)

- **Storage:** Previews live in a **dedicated cache folder** under the system temp area, scoped to this app (so they do not clutter the user’s Documents folder).
- **Naming:** For each original `name.ext`, previews are named with **width suffixes**, e.g. `name_350.ext`, `name_512.ext`, `name_1024.ext` in that cache folder.
- **Deletion:** When a slide is removed from the working set, **delete** its preview files if they exist.
- **Regeneration:** Same as add flow when originals exist but previews are missing.

---

## 8. Export file specification (CSV)

### 8.1 Format

- **Delimiter:** semicolon **`;`**
- **Encoding:** UTF-8 (recommended; match reference if interoperability is required).
- **Columns (in order):**
  `file`, `jury`, `subj_id`, `subj_name`, `coeff`, `version`, `transform_id`

### 8.2 Which slides become rows

Include **only** slides whose category is **Fixed** or **Jury** (i.e. **not** Excluded).

### 8.3 Field rules

| Column | Rule |
|--------|------|
| `file` | Slide’s **file name without extension** (strip extension only). |
| `jury` | If category is **Jury**, the literal **`X`**; otherwise **empty**. |
| `subj_id` | Species id from catalog. |
| `subj_name` | Species common name from catalog. |
| `coeff` | Coefficient from catalog. |
| `version` | Version from catalog. |
| `transform_id` | Integer **`0`–`7`**: **canonical** orientation of the slide relative to the original file on disk (§3.5). Encodes the **deduplicated** result of all user transforms (rotation / flips), not the operation history. Meaning of each value is **fixed** in §8.6. Use **`0`** when there is no net transform. |

### 8.4 Validation before write

1. **No duplicate species among exported rows:** Each exported row’s `subj_id` must appear **at most once** (consistent with **§3.3**: each species at most once across **Jury ∪ Fixed**). If two rows would share the same `subj_id`, **abort** and show:
   **“Sono presenti specie doppie.”**
2. **Every exported row must have a species:** `subj_id` must be **non-zero** and valid. If any exported row would have no species, **abort** and show:
   **“Non a tutte le slide è stata assegnata una specie.”**

### 8.5 Success feedback

After a successful save, show a short confirmation (Italian), e.g. that the file was saved.

### 8.6 Canonical `transform_id` values (interoperability)

These eight values enumerate **D₄** in a **fixed** convention so downstream tools can interpret the column without ambiguity.

**Encoding:** `transform_id = 4f + k` with **`f ∈ {0,1}`** and **`k ∈ {0,1,2,3}`**. Let **R** = rotate **90° clockwise** around the image center. Let **H** = horizontal flip (mirror left–right). For each id, the **net** transform applied to the **original** bitmap (as in the file) is **H^f ∘ R^k**: apply **R** **k** times, then **H** if **f = 1** (same as **H(R^k(·))**).

| `transform_id` | `f` | `k` | Result (relative to original file) |
|----------------|-----|-----|-------------------------------------|
| `0` | 0 | 0 | Identity (no transform) |
| `1` | 0 | 1 | **R** (90° CW) |
| `2` | 0 | 2 | **R²** (180°) |
| `3` | 0 | 3 | **R³** (270° CW) |
| `4` | 1 | 0 | **H** (horizontal flip only) |
| `5` | 1 | 1 | **H ∘ R** |
| `6` | 1 | 2 | **H ∘ R²** |
| `7` | 1 | 3 | **H ∘ R³** |

**Dedup rule:** User gesture sequences (any order of 90° rotations and flips) **update** this canonical pair **(f, k)** by group composition in **D₄**; the exported **`transform_id`** is always the **reduced** value in this table.

**Note:** Implementations may represent state internally with matrices or quaternions; the **CSV** must use the **integer 0–7** above. **Vertical flip** as a separate control maps to whichever **id** produces that pixel arrangement.

**Legacy consumers:** Older pipelines expecting only six columns may ignore the seventh column or read by header name `transform_id`.

---

## 9. Persistence (session to session)

**Must persist:**

- Ordered list of slides (references to originals).
- Each slide’s **category**, **species id**, and **orientation metadata** (rotation / flips per §3.5).
- Enough data to **rebuild or verify** previews (or regenerate from originals).

**Need not persist:**

- Active **filter tab** (always start on “All” or equivalent default).
- **Modal** open/closed state.
- **Selection** state.

The **species usage overview** (§5.8, §10.6) is **derived** from slides + catalog and needs no separate persisted state.

---

## 10. UI structure (behavioral, not visual framework)

### 10.1 Top region

- **Default mode (nothing selected):** Application title and **Export CSV**; export enabled only if there is at least one slide.
- **Selection mode:** Toolbar for bulk actions (§4.6).

### 10.2 Tabs

- Four tabs with **counts**: All, Excluded, Fixed, Jury.

### 10.3 Main content

- **Empty:** instructional text + load control.
- **With slides:** grid + optional floating **load more** control when nothing is selected.

### 10.4 Window behavior

- Reasonable default and minimum window size so the grid remains usable.
- **Browser-style** drag-and-drop on the window should **not** hijack navigation; the reference disables default drop behavior (implementations may still add explicit “drop files here” later—**not** required by this PRD).

### 10.5 Help / about

- A way to open the project’s public page (URL in the reference repo) from the menu is acceptable.

### 10.6 Species usage overview

- A **full catalog list** (§5.8) is available in the UI (exact placement—sidebar, panel, or separate view—is an implementation detail) so participants can see **all species** and which they have already used for **Jury** (green), **Fixed** (yellow), or none (unhighlighted).

---

## 11. Edge cases and product decisions

| Situation | Expected behavior |
|-----------|-------------------|
| Same file name added twice | One slide remains (last wins). |
| Original file deleted on disk | Display may break; regeneration cannot fix missing originals—acceptable to show broken state or error (reference focuses on missing **previews**). |
| Export with zero eligible slides | Either disable export or export empty—reference ties export to “has any slide”; **validation** still applies to **eligible** rows only. |
| User exports with duplicate species | Block with Italian error (§8.4). |
| Interim invalid assignments (e.g. duplicate `subj_id` among **Jury ∪ Fixed** before the user fixes them) | **Validation at export** is required (§8.4). **Optional enhancement:** warn or block when assigning a species already used on another non-excluded slide. |
| Same species on multiple **Excluded** slides | Allowed; excluded rows are not exported. |
| **`transform_id`** on export | Always one of **`0`–`7`**; derived from the slide’s **canonical** transform (§8.6), not from raw gesture history. |

---

## 12. Non-goals (out of scope for this PRD)

- Cloud sync, multi-user collaboration, or server-side storage.
- **Advanced editing:** crop, retouch, localized corrections, or **color grading** (orientation changes in §3.5 / §5.7 are **in scope**).
- Replacing the official species list with user-defined lists (unless you explicitly extend the product).
- Mobile or web-first delivery (desktop remains the target).

---

## 13. Acceptance criteria (high level)

The implementation is **acceptable** if a participant can:

1. Load multiple images, see them in a responsive grid with previews.
2. Assign categories and species, filter, and (on “All”) reorder.
3. Use **rotation** and **mirror/flip** on slides without overwriting originals by default, with previews and persistence matching §3.5, §5.7, and §9.
4. Open the **full species list** (§5.8, §10.6) and see **green** / **yellow** / **unhighlighted** rows consistent with current **Jury** / **Fixed** / unused usage.
5. Save work, restart the app, and see **the same slides, order, categories, species, and orientation metadata** (§9).
6. Export a **`;`-separated CSV** that matches §8 when validation passes—including column **`transform_id`** (§8.3, §8.6) for each exported row—and see **exact** Italian errors from §8.4 when validation does not.
7. Install and run the app on **macOS, Windows, and Linux** from **standalone** distributables that **minimize** separate runtime or dependency installation steps for non-technical users (§14).

---

## 14. Implementation and distribution constraints

These constraints apply **in addition** to functional requirements elsewhere. They do **not** mandate a single named framework; they describe **what** the chosen stack must deliver.

### 14.1 Platforms

- Ship as a **desktop** application for **macOS**, **Windows**, and **Linux** with the **same core product behavior** on each platform (§5.1).

### 14.2 Imaging performance and libraries

- Use languages and libraries suited to **high-resolution** images: decoding, scaling, and caching must remain responsive for typical competition photo sizes.
- Thumbnail and preview behavior must align with **§5.2** and **§7**; simple transforms (**§5.7**) must feel **fast** on all supported OSes.

### 14.3 Distribution (non-technical users)

- Prefer **standalone** installers or portable bundles (per platform) so participants are **not** required to install interpreters, runtimes, or system packages manually before first use.
- Exact packaging (signed `.dmg`, `.msi`, AppImage, etc.) is left to implementation; the **goal** is minimal friction for non-technical users.

### 14.4 Choosing a stack

- Favor ecosystems with **mature cross-platform desktop packaging**, **native or well-optimized image pipelines**, and a path to **single-download** installs. Non-binding examples include **Rust, C++, C#/.NET, or other** stacks that meet §14.1–§14.3—**evaluation against these criteria** matters more than the language name.

### 14.5 Reference codebase

The **reference implementation** in the repository is **one** possible approach; it is **not** a required architecture. Where this PRD is silent, implementations may differ if **§13** and **§14** are satisfied.
