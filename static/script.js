for (const el of document.getElementsByTagName("time")) {
  const stamp = new Date(el.getAttribute("datetime"));
  el.innerHTML = stamp.toLocaleString(navigator.language);
}
