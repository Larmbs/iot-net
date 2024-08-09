const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);

document.addEventListener("DOMContentLoaded", async function () {
  let device_id = urlParams.get("device_id");

  let device_info = await fetch("/api/post/device", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id: device_id,
    }),
  }).then((response) => {
    return response.json();
  });
  console.log(device_info.sensors)
  let sensors_html = device_info.sensors.map((name) => 
    `
        <h2>${name}</h2>
    `
  ).join("");
  console.log(sensors_html);
  document
    .getElementById("device-sensors")
    .insertAdjacentHTML("afterend", sensors_html);
});
