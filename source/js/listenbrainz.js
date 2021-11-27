window.addEventListener('DOMContentLoaded', async () => {
  const result = await window.fetch('https://api.listenbrainz.org/1/user/BaukeXYZ/playing-now');
  if (!result.ok) {
    console.warn(result.status);
    return;
  }

  const data = await result.json();
  if (data.payload.listens.length === 0) {
    return;
  }

  const listen = data.payload.listens[0];
  const listenHtml = `<div class="divider"></div><div class="listenbrainz">
  <a href="https://listenbrainz.org/user/BaukeXYZ/" target="_blank">
    <img src="https://listenbrainz.org/static/img/logo_big.svg">
    ${listen.track_metadata.artist_name} - ${listen.track_metadata.track_name}
  </a>
</div>`;

  document.querySelector('.page-main').insertAdjacentHTML('beforeend', listenHtml);
});
