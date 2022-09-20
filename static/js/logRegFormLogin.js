document.getElementById("logRegForm").addEventListener("submit", async (e) => {
  e.preventDefault();
  let errors = ["email", "password"];
  const feedbackListener = () => {
    errors.forEach((field) => {
      document.getElementById(`${field}`).classList.remove("is-invalid");
      document.getElementById(`validation_${field}`).innerText = "";
      document
        .getElementById(`${field}`)
        .removeEventListener("click", feedbackListener);
    });
  };
  errors.forEach((field) => {
    document.getElementById(`validation_${field}`).innerText = "";
  });

  let formData = new FormData(e.target);
  let body = JSON.stringify(Object.fromEntries(formData));

  async function postForm(body) {
    const req = await fetch("/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body,
    });
    const res = await req.json();
    return res;
  }

  let response = await postForm(body);

  if (response.status !== 200) {
    //all from struct level validation
    if ("__all__" in response && Object.keys(response).length === 1) {
      errors.forEach((field) => {
        document.getElementById(`${field}`).classList.add("is-invalid");
        document.getElementById(`validation_${field}`).innerText =
          response.__all__[0].message;
        document
          .getElementById(`${field}`)
          .addEventListener("click", feedbackListener);
      });
    } else {
      errors.forEach((field) => {
        if (response.hasOwnProperty(field) === false) return;
        if (response[field].length < 1) return;

        document.getElementById(`${field}`).classList.add("is-invalid");
        response[field].forEach((err) => {
          if (err.message === null) return;
          document.getElementById(
            `validation_${field}`
          ).innerText += `${err.message}.\xA0`;
        });

        document
          .getElementById(`${field}`)
          .addEventListener("click", feedbackListener);
      });
    }
  }
});
