import("../pkg/index.js")
  .then((mod) => {
    const form = document.getElementById("presentation-input");
    const textArea = form.querySelector("textarea");
    const presentation = document.getElementById("presentation");

    const pt = mod.PresentationTool.new(form, textArea, presentation);

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      pt.start_presentation();
    });
  })
  .catch(console.error);
