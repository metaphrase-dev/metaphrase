import { defineComponent } from "vue";

function createIcon(name) {
  return {
    name,
    render() {
      return (
        <svg width="24px" height="24px" viewBox="0 0 24 24">
          {/* Those defaut widths and heights, as any other attributes, can be overridden on call site. */}
          <path d={ICONS_PATHS[name]} fill="currentColor" />
        </svg>
      );
    },
  };
}

// All the icons "path" definitions, in alphabetical order.
// Prettier formatting is ignored on this object to ensure each path
// is defined on one and only line for readability purposes.
// prettier-ignore
const ICONS_PATHS = {
  IconCheck: "M21 7L9 19l-5.5-5.5l1.41-1.41L9 16.17L19.59 5.59L21 7z",
  IconClose: "M20 6.91L17.09 4L12 9.09L6.91 4L4 6.91L9.09 12L4 17.09L6.91 20L12 14.91L17.09 20L20 17.09L14.91 12L20 6.91z",
  IconPlus: "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
  IconChevronRight: "M8.59 16.58L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.42z",
  IconFile: "M13 9V3.5L18.5 9M6 2c-1.11 0-2 .89-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6H6z",
  IconFolder: "M10 4H4c-1.11 0-2 .89-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-8l-2-2z",
  IconKey: "M7 14a2 2 0 0 1-2-2a2 2 0 0 1 2-2a2 2 0 0 1 2 2a2 2 0 0 1-2 2m5.65-4A5.99 5.99 0 0 0 7 6a6 6 0 0 0-6 6a6 6 0 0 0 6 6a5.99 5.99 0 0 0 5.65-4H17v4h4v-4h2v-4H12.65z",
  IconLock: "M12 17a2 2 0 0 0 2-2a2 2 0 0 0-2-2a2 2 0 0 0-2 2a2 2 0 0 0 2 2m6-9a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V10a2 2 0 0 1 2-2h1V6a5 5 0 0 1 5-5a5 5 0 0 1 5 5v2h1m-6-5a3 3 0 0 0-3 3v2h6V6a3 3 0 0 0-3-3z",
  IconLogin: "M19 3H5c-1.11 0-2 .89-2 2v4h2V5h14v14H5v-4H3v4a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m-8.92 12.58L11.5 17l5-5l-5-5l-1.42 1.41L12.67 11H3v2h9.67l-2.59 2.58z",
  IconLogout: "M14.08 15.59L16.67 13H7v-2h9.67l-2.59-2.59L15.5 7l5 5l-5 5l-1.42-1.41M19 3a2 2 0 0 1 2 2v4.67l-2-2V5H5v14h14v-2.67l2-2V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5c0-1.11.89-2 2-2h14z",
  IconSearch: "M9.5 3A6.5 6.5 0 0 1 16 9.5c0 1.61-.59 3.09-1.56 4.23l.27.27h.79l5 5l-1.5 1.5l-5-5v-.79l-.27-.27A6.516 6.516 0 0 1 9.5 16A6.5 6.5 0 0 1 3 9.5A6.5 6.5 0 0 1 9.5 3m0 2C7 5 5 7 5 9.5S7 14 9.5 14S14 12 14 9.5S12 5 9.5 5z",
  IconTrash: "M9 3v1H4v2h1v13a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V6h1V4h-5V3H9m0 5h2v9H9V8m4 0h2v9h-2V8z",
  IconUser: "M12 4a4 4 0 0 1 4 4a4 4 0 0 1-4 4a4 4 0 0 1-4-4a4 4 0 0 1 4-4m0 10c4.42 0 8 1.79 8 4v2H4v-2c0-2.21 3.58-4 8-4z",
  IconWarning: "M13 14h-2V9h2m0 9h-2v-2h2M1 21h22L12 2L1 21z",
}

// Exported icons, in alphabetical order.
// For Vite to do its exporting/translating job correctly, lines have to
// contains the `defineComponent`; the `createIcon` is a way to shorten it.
export const IconCheck = defineComponent(createIcon("IconCheck"));
export const IconChevronRight = defineComponent(createIcon("IconChevronRight"));
export const IconClose = defineComponent(createIcon("IconClose"));
export const IconFile = defineComponent(createIcon("IconFile"));
export const IconFolder = defineComponent(createIcon("IconFolder"));
export const IconKey = defineComponent(createIcon("IconKey"));
export const IconLock = defineComponent(createIcon("IconLock"));
export const IconLogin = defineComponent(createIcon("IconLogin"));
export const IconLogout = defineComponent(createIcon("IconLogout"));
export const IconPlus = defineComponent(createIcon("IconPlus"));
export const IconSearch = defineComponent(createIcon("IconSearch"));
export const IconTrash = defineComponent(createIcon("IconTrash"));
export const IconUser = defineComponent(createIcon("IconUser"));
export const IconWarning = defineComponent(createIcon("IconWarning"));
