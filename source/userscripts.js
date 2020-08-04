const {promises: fsp} = require('fs');
const nunjucks = require('nunjucks');
const {join} = require('path');

const {readScriptMetadata, scripts} = require('userscripts');

async function main() {
  // Make sure the `public/userscripts` directory exists.
  await fsp.mkdir(join(__dirname, '../public/userscripts/'), {recursive: true});

  // Configure Nunjucks to use the current directory for templates.
  nunjucks.configure(__dirname, {
    lstripBlocks: true,
    trimBlocks: true,
    throwOnUndefined: true
  });

  // Create an array to hold all the script metadatas we'll use to render the template.
  const scriptMetadatas = [];

  // Define the source directory where the userscripts are located.
  const sourceDir = join(__dirname, '../node_modules/userscripts/source/');

  for (const script of scripts) {
    // Read the script metadata and add it to our array.
    const metadata = await readScriptMetadata(script);

    // The way the metadata is parsed is inside arrays, so extract
    // all of them so we don't have to in the template.
    metadata.description = metadata.description[0];
    metadata.downloadURL = metadata.downloadURL[0];
    metadata.name = metadata.name[0];
    metadata.version = metadata.version[0];
    // Remove the `https://` and trailing `/*` from the domain.
    metadata.domain = metadata.match[0].slice(
      metadata.match[0].indexOf('://') + 3,
      metadata.match[0].lastIndexOf('/')
    );

    scriptMetadatas.push(metadata);

    // Define the filename for the script.
    const filename = `${script}.user.js`;

    // Copy all the userscripts to the `public/userscripts/` directory.
    await fsp.copyFile(
      join(sourceDir, filename),
      join(__dirname, '../public/userscripts/', filename)
    );
  }

  // Render the userscripts template and write it to file.
  await fsp.writeFile(
    join(__dirname, '../public/userscripts/index.html'),
    nunjucks.render('userscripts/index.html', {scripts: scriptMetadatas})
  );
}

module.exports = {
  build: main
};

// Run `main()` if this script was called directly (like `node source/userscripts.js`).
if (require.main === module) {
  main();
}
