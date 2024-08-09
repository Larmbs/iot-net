document.addEventListener("DOMContentLoaded", async function () {
  let device_list = document.getElementById("device-list");

  let device_id_map = await fetch("/api/get/devices", { method: "GET" }).then(
    (response) => {
      return response.json();
    }
  );
  for (let name in device_id_map.devices) {
    let device_details = await fetch("/api/post/device", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: device_id_map.devices[name],
      }),
    }).then((response) => {
      return response.json();
    });

    device_list.insertAdjacentHTML(
      "afterend",
      `
      <div onclick='window.location.href = "/tracker?device_id=${device_id_map.devices[name]}";' class="device-card">
        <h3>${name}</h3>
        <p>${device_details.description}</p>
      </div>
      `
    );
  }
});
