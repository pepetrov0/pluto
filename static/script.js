const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

// auto-localize dates
for (const el of document.getElementsByTagName("time")) {
  if (el.dataset.localized) continue;
  el.dataset.localized = true;

  const stamp = new Date(el.getAttribute("datetime"));
  el.innerHTML = stamp.toLocaleString(navigator.language);
}

// auto-populate timezone fields
for (const el of document.getElementsByClassName("input timezone")) {
  if (el.dataset.populated) continue;
  el.dataset.populated = true;

  for (const ch of el.getElementsByTagName("option")) {
    if (ch.value === timezone) {
      ch.selected = true;
      break;
    }
  }
}
