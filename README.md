# grapheme_clusters_wasm

## Example

1. Run `wasm-pack build --target web`
1. Create `index.html`
   ```html
   <!DOCTYPE html>
   <html>

   <head>
     <meta charset="utf-8">
     <title>Example</title>
   </head>

   <body>
     <script type="module">
       import init, { grapheme_clusters } from "./pkg/grapheme_clusters_wasm.js";
       init().then(() => {
         console.debug(grapheme_clusters("か゚き゚く゚け゚こ゚カ゚キ゚ク゚ケ゚コ゚"));
       });
     </script>
   </body>

   </html>
   ```
1. Run `python3 -m http.server 8080`
