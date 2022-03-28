window.addEventListener('DOMContentLoaded', async () => {
  document.querySelector('.page-main').insertAdjacentHTML('beforeend', '<div class="divider"></div>');

  const loop = async () => insertHtml(await getCurrentListen());
  await loop();

  setInterval(loop, 60 * 1000);
});

async function getCurrentListen() {
  const result = await window.fetch('https://api.listenbrainz.org/1/user/BaukeXYZ/playing-now');
  if (!result.ok) {
    console.warn(result.status);
    return null;
  }

  const data = await result.json();
  if (data.payload.listens.length === 0) {
    return null;
  }

  return data.payload.listens[0];
}

function insertHtml(listen) {
  if (listen === null) {
    return;
  }

  const existing = document.querySelector('.listenbrainz');
  if (existing !== null) {
    existing.remove();
  }

  const listenHtml = `<p class="listenbrainz">
    <img src="https://listenbrainz.org/static/img/logo_big.svg">
    <span class="byline">Currently listening to</span>
    <a href="https://listenbrainz.org/user/BaukeXYZ/" target="_blank">
      ${listen.track_metadata.artist_name} - ${listen.track_metadata.track_name}
    </a>
</p>`;

  document.querySelector('.page-main').insertAdjacentHTML('beforeend', listenHtml);
}
