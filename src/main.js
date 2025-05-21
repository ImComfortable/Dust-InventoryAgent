const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function enviarJustificativa(motivo) {
  try {
    await invoke("register_inactivity", { justificativa: motivo });
  } catch (error) {
    console.error("Erro ao enviar justificativa:", error);
    greetMsgEl.textContent = "Erro ao enviar justificativa.";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#custom-reason");
  greetMsgEl = document.createElement("p");
  greetMsgEl.id = "greet-msg";
  greetInputEl.parentNode.appendChild(greetMsgEl);

  document.querySelector("#inactivity-form").addEventListener("submit", (e) => {
    e.preventDefault();
    const motivo = greetInputEl.value.trim();
    if (motivo) {
      enviarJustificativa(motivo);
    } else {
      greetMsgEl.textContent = "Por favor, insira ou selecione um motivo.";
    }
  });

  
  document.querySelectorAll(".option-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      const motivo = btn.textContent.trim();
      enviarJustificativa(motivo);
    });
  });
});
