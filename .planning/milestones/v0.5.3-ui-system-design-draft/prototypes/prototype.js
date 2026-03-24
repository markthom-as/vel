function toggleById(id) {
  const el = document.getElementById(id);
  if (!el) return;
  const next = el.getAttribute("aria-hidden") !== "false";
  el.setAttribute("aria-hidden", String(!next));
}

document.addEventListener("click", (event) => {
  const target = event.target;
  if (!(target instanceof HTMLElement)) return;
  const id = target.getAttribute("data-toggle");
  if (id) toggleById(id);
});
