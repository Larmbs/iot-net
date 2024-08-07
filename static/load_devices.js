document.addEventListener("DOMContentLoaded", async function () {
  let device_list = document.getElementById("device-list");

  let device_id_map = await fetch("/api/get/devices", {method: "GET"}).then((response) => {
    console.log(response);
    response.json()
  }
  );

  device_id_map.devices.forEach(async (id, name) => {
    let deviceDetails = await fetch("/api/get/device", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: { "id": id },
    }).then((response) => response.json());

    device_list.insertAdjacentHTML(
      "beforeend",
      `
        <div class="device-card">
            <h3>${name}</h3>
            <p>${deviceDetails.description}</p>
        </div>`
    );
  });
});
