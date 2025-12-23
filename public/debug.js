(function () {
  var container = document.createElement("div");
  container.id = "debug-console";
  container.style.cssText =
    "position:fixed;top:0;left:0;width:100%;height:300px;overflow:auto;background:rgba(0,0,0,0.8);color:lightgreen;z-index:99999;font-family:monospace;font-size:12px;padding:5px;";

  // Perform cleanup if it exists (HMR)
  var old = document.getElementById("debug-console");
  if (old) old.remove();

  document.body.appendChild(container);

  function logToParams(args, color) {
    var msg = Array.from(args)
      .map((a) => {
        if (typeof a === "object") return JSON.stringify(a);
        return String(a);
      })
      .join(" ");
    var el = document.createElement("div");
    el.style.color = color || "white";
    el.innerText = msg;
    container.appendChild(el);
    container.scrollTop = container.scrollHeight;
  }

  var originalLog = console.log;
  console.log = function () {
    originalLog.apply(console, arguments);
    logToParams(arguments, "white");
  };

  var originalWarn = console.warn;
  console.warn = function () {
    originalWarn.apply(console, arguments);
    logToParams(arguments, "yellow");
  };

  var originalError = console.error;
  console.error = function () {
    originalError.apply(console, arguments);
    logToParams(arguments, "red");
  };

  window.addEventListener("error", function (e) {
    logToParams(
      ["[Uncaught]", e.message, "at", e.filename, ":", e.lineno],
      "red"
    );
  });
})();
console.log("Debug console initialized");
