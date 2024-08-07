document.addEventListener("DOMContentLoaded", async function () {
  let device_list = document.getElementById("device-list");

  let device_id_map = await fetch("/api/get/devices", { method: "GET" }).then(
    (response) => {
      console.log(response);
      return response.json();
    }
  );
  console.log(device_id_map.devices);
  for (let name in device_id_map.devices) {
    let device_details = await fetch("/api/post/device", {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json'
            },
          body: JSON.stringify({
                id: device_id_map.devices[name],
          })
        }).then((response) => {
      console.log(response);
      return response.json();
    });
    console.log(device_details)
    device_list.insertAdjacentHTML("afterend",
      `<div class="device-card">
        <h3>${name}</h3>
        <p>${device_details.description}</p>
    </div>`
    );
  }
});
