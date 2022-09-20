document.getElementById("logRegForm").addEventListener("submit", async (e) => {
  e.preventDefault();
  let errors = [
    "first_name",
    "last_name",
    "email",
    "password",
    "confirm_password",
  ];
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
    const req = await fetch("/register", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body,
    });
    const res = await req.json();
    return res;
  }

  let response = await postForm(body);

  if (response.status !== 201) {
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
});
