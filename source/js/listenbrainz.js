window.addEventListener('DOMContentLoaded', async () => {
  document.querySelector('.page-main').insertAdjacentHTML('beforeend', '<div class="divider"></div>');

  const loop = async () => {
    const listen = await getCurrentListen();
    const image = await getCoverArt(listen);
    insertHtml(listen, image);
  };

  await loop();

  setInterval(loop, 5 * 60 * 1000);
});

async function getCoverArt(listen) {
  const releaseMbid = listen.track_metadata.additional_info?.release_mbid;
  if (releaseMbid === undefined) {
    return;
  }

  const result = await window.fetch(`https://coverartarchive.org/release/${releaseMbid}`, {
    headers: {
      Accept: 'application/json',
    },
  });
  if (!result.ok) {
    return;
  }

  const data = await result.json();
  if (data.images[0]?.thumbnails === undefined) {
    return;
  }

  const thumbnails = data.images[0].thumbnails;
  return thumbnails.small ?? thumbnails['250'] ?? thumbnails['500'] ?? undefined;
}

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

function insertHtml(listen, image) {
  if (listen === null) {
    return;
  }

  const existing = document.querySelector('.listenbrainz');
  if (existing !== null) {
    existing.remove();
  }

  const text = `${listen.track_metadata.artist_name} - ${listen.track_metadata.track_name}`;
  const alt = image === undefined ? 'ListenBrainz Logo' : `${text} Cover Art`;

  image = image ?? 'https://listenbrainz.org/static/img/logo_big.svg';

  const listenHtml = `<p class="listenbrainz">
    <img alt="${alt}" title="${alt}" src="${image}">
    <span class="byline">Currently listening to</span>
    <a href="https://listenbrainz.org/user/BaukeXYZ/" target="_blank">
      ${text}
    </a>
</p>`;

  document.querySelector('.page-main').insertAdjacentHTML('beforeend', listenHtml);
}
