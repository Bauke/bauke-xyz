const {promises: fsp} = require('fs');
const nunjucks = require('nunjucks');
const {join} = require('path');

const {build} = require('../submodules/userstyles/source');
const {
  readStyleMetadata
} = require('../submodules/userstyles/source/utilities');

async function main() {
  // Make sure the `public/userstyles` directory exists.
  await fsp.mkdir(join(__dirname, '../public/userstyles/'), {recursive: true});

  // Configure Nunjucks to use the current directory for templates.
  nunjucks.configure(__dirname, {
    lstripBlocks: true,
    trimBlocks: true,
    throwOnUndefined: true
  });

  // Build the userstyles.
  await build();

  // Define the build directory where the userstyles will be built to.
  const buildDir = join(__dirname, '../submodules/userstyles/build/');

  // Read and sort the files in the build directory.
  const styles = await fsp.readdir(buildDir);
  styles.sort((a, b) => a.localeCompare(b));

  // Create an array to hold all the style metadatas we'll use to render the template.
  const styleMetadatas = [];

  for (const style of styles) {
    // Remove the `.user.css` so we get just the style name.
    const styleName = style.slice(0, style.indexOf('.'));

    // Read the style metadata and add it to our array.
    styleMetadatas.push(await readStyleMetadata(styleName));

    // Copy all the built userstyles to the `public/userstyles/` directory.
    await fsp.copyFile(
      join(buildDir, style),
      join(__dirname, '../public/userstyles/', style)
    );
  }

  // Render the userstyles template and write it to file.
  await fsp.writeFile(
    join(__dirname, '../public/userstyles/index.html'),
    nunjucks.render('userstyles/index.html', {styles: styleMetadatas})
  );
}

// Run `main()` if this script was called directly (like `node source/userstyles.js`).
if (require.main === module) {
  main();
}
