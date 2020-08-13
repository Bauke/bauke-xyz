const cpy = require('cpy');

const {build: buildUserscripts} = require('./userscripts');
const {build: buildUserstyles} = require('./userstyles');

async function main() {
  console.log('Copying files.');
  await cpy(
    [
      '**',
      '!index.js',
      '!userscripts.js',
      '!userstyles.js',
      '!userscripts/*',
      '!userstyles/*',
      '!scss/*'
    ],
    '../public/',
    {
      cwd: 'source',
      parents: true
    }
  );

  await cpy(
    ['node_modules/modern-normalize/modern-normalize.css'],
    'public/css/'
  );

  console.log('Building userscripts.');
  await buildUserscripts();

  console.log('Building userstyles.');
  await buildUserstyles();
}

// Run `main()` if this script was called directly (like `node source/index.js`).
if (require.main === module) {
  main();
}
