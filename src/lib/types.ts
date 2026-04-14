export type Category = "excluded" | "fixed" | "jury";

export type FilterTab = "all" | Category;

export interface ThumbnailPaths {
  s350: string;
  s512: string;
  s1024: string;
}

export interface Slide {
  id: string;
  path: string;
  width: number;
  height: number;
  category: Category;
  subjectId: number;
  transformId: number;
  thumbnails: ThumbnailPaths;
  /** EXIF orientation tag 1–8; default 1 (normal). */
  exifOrientation?: number;
  /** True until the preview cache files are written (background generation). */
  thumbnailsPending?: boolean;
  selected?: boolean;
}

export interface Species {
  id: number;
  commonName: string;
  scientificName: string;
  coefficient: number;
  version: string;
}

export interface PersistedSlide {
  id: string;
  path: string;
  category: Category;
  subjectId: number;
  transformId: number;
  exifOrientation?: number;
}

export interface AppState {
  slides: PersistedSlide[];
}

export type ThemePreference = "light" | "dark" | "system";
