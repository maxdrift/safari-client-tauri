import type { Category } from "$lib/types";

export function categoryLabel(c: Category): string {
  if (c === "excluded") return "Scartata";
  if (c === "fixed") return "Punt. fisso";
  return "In giuria";
}

export const I18N = {
  appTitle: "Safari Client",
  loadImages: "Carica immagini",
  exportCsv: "Esporta CSV",
  importCsv: "Importa CSV",
  competitorExportTitle: "Nome concorrente",
  competitorExportHint:
    "Il nome viene usato nel nome file (es. scheda_Mario_Rossi.csv). Puoi modificarlo a ogni esportazione.",
  competitorNamePlaceholder: "Nome e cognome",
  importCsvSuccess: (n: number) =>
    n === 1 ? "Aggiornata 1 slide dal CSV." : `Aggiornate ${n} slide dal CSV.`,
  importCsvNone: "Nessuna slide corrisponde ai nomi file nel CSV.",
  importCsvErrorTitle: "Importazione CSV non riuscita",
  emptyHint: "Carica le foto per iniziare.",
  all: "Tutte",
  excluded: "Scartate",
  fixed: "Punt. fisso",
  jury: "In giuria",
  selectedOne: (n: number) => (n === 1 ? "1 slide selezionata" : `${n} slide selezionate`),
  deselectAll: "Deseleziona tutto",
  selectAll: "Seleziona tutto",
  judgeMenu: "Giudica",
  assignSpecies: "Assegna specie",
  delete: "Elimina",
  clearSelection: "Rimuovi selezione",
  speciesModalTitle: "Seleziona specie",
  speciesSearch: "Cerca per nome comune o scientifico…",
  speciesOverview: "Elenco specie",
  removeSpecies: "Rimuovi specie",
  confirmDelete: "Eliminare le slide selezionate?",
  exportErrorTitle: "Impossibile esportare la scheda concorrente",
  exportSuccess: "File salvato correttamente.",
  cancel: "Annulla",
  confirm: "Conferma",
  rotateCw: "Ruota ↻",
  rotateCcw: "Ruota ↺",
  flipH: "Specchia H",
  flipV: "Specchia V",
  close: "Chiudi",
  loadingImages: "Caricamento immagini…",
  prevImage: "Immagine precedente",
  nextImage: "Immagine successiva",
  zoomFit: "Adatta allo schermo",
  zoom100: "100% (dimensioni reali)",
  zoomWheelHint: "Rotella: zoom · Due dita / Ctrl+rotella: zoom · Trascina per spostare se ingrandito",
  speciesOnTile: "Specie",
  gridZoom: "Zoom griglia",
  lightboxDimensions: "Dimensioni",
  lightboxCategory: "Categoria",
  lightboxSelection: "Selezione",
  lightboxSelected: "Selezionata",
  lightboxNotSelected: "Non selezionata",
  lightboxSpecies: "Specie",
  lightboxScientific: "Nome scientifico",
  lightboxCoefficient: "Coefficiente",
  lightboxCatalogVersion: "Catalogo",
  lightboxNoSpecies: "Nessuna specie assegnata",
  theme: "Tema",
  themeLight: "Chiaro",
  themeDark: "Scuro",
  themeSystem: "Sistema",
  settings: "Impostazioni",
  settingsTitle: "Impostazioni",
  settingsSectionSpecies: "Elenco specie (catalogo)",
  settingsSectionExport: "Esportazione scheda (CSV)",
  settingsSpeciesBlurb:
    "L’elenco delle specie di pesce usato quando assegni una specie alle foto e quando importi un CSV delle slide. Non è l’importazione delle slide: quella resta da «Importa CSV» nella barra.",
  settingsSpeciesFormatHint:
    "Formato file: una riga per specie, colonne separate da punto e virgola: id;nome_comune;nome_scientifico;coefficiente;versione (come l’elenco predefinito).",
  settingsSpeciesImport: "Importa CSV elenco…",
  settingsSpeciesRestore: "Ripristina elenco predefinito",
  settingsSpeciesImportSuccess: "Elenco specie importato correttamente.",
  settingsSpeciesImportErrorTitle: "Importazione elenco specie non riuscita",
  settingsSpeciesRestoreConfirm:
    "Ripristinare l’elenco specie fornito con l’app? L’elenco personalizzato verrà rimosso.",
  settingsSpeciesOrphanWarning: (ids: number[]) =>
    `Attenzione: alcune foto usano ID specie non presenti nel nuovo elenco: ${ids.join(", ")}. Riassegna le specie se necessario.`,
  settingsExportNameHint:
    "Nome predefinito per il file scheda_….csv quando esporti la scheda concorrente (dati delle slide). Separato dall’elenco specie sopra.",
} as const;
