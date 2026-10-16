// Bundle Coriolis's calculation modules (third-party checkout, read as data) with entry.js.
// Usage: node build.mjs [coriolis-ref dir]   -> out.js
import * as esbuild from 'esbuild';
import fs from 'fs';
import path from 'path';
const H = path.dirname(new URL(import.meta.url).pathname);
const REF = process.argv[2] || '/tmp/coriolis-ref';
// Calculations.js calls its sibling exports as `this.fn`, relying on the bundler keeping the
// module namespace as `this` (webpack does; esbuild inlines namespace calls). Route them
// through an object holding the module's exports.
const selfCalls = {
  name: 'calc-self-calls',
  setup(b) {
    b.onLoad({ filter: /shipyard[\\/]Calculations\.js$/ }, (a) => {
      const src = fs.readFileSync(a.path, 'utf8');
      const names = [...src.matchAll(/^export function (\w+)/gm)].map((m) => m[1]);
      return {
        contents: src.replace(/\bthis\.(?=[A-Za-z_]+\()/g, '__calc.') + `\nconst __calc = { ${names.join(', ')} };\n`,
        loader: 'jsx',
      };
    });
  },
};
await esbuild.build({
  entryPoints: [path.join(H, 'entry.js')], bundle: true, platform: 'node', format: 'cjs',
  loader: { '.js': 'jsx' }, logLevel: 'error', outfile: path.join(H, 'out.js'),
  alias: { 'coriolis-data/dist': path.join(H, 'coriolis-data-dist.js'), coriolis: path.join(REF, 'coriolis') },
  external: ['zlib', 'react', 'lodash', 'lodash/lang', 'lz-string'],
  plugins: [selfCalls],
});
