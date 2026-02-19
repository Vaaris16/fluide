| Framework | Global Sass file     | Imported / Wired in | Notes                  |
| --------- | -------------------- | ------------------- | ---------------------- |
| Vanilla   | src/style.scss       | main.js / main.ts   | Manual import required |
| React     | src/index.scss       | main.tsx            | Manual import required |
| Preact    | src/index.scss       | main.tsx            | Same as React          |
| Lit       | src/index.scss       | main.ts             | Global styles          |
| Vue       | src/assets/main.scss | main.js             | Convention-based       |
| Svelte    | src/app.scss         | main.ts             | Global stylesheet      |
| Solid     | src/index.scss       | main.tsx            | Similar to React       |
| Qwik      | src/global.scss      | Auto-wired          | Framework handles it   |
| Angular   | src/styles.scss      | angular.json        | Not Vite-based         |
| Ember     | app/styles/app.scss  | Auto-wired          | Not Vite-based         |
| Marko     | src/style.scss       | Auto-wired          | Framework handles it   |
