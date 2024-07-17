#include <WiFi.h>
#include <HTTPClient.h>

// Replace with your network credentials
const char* ssid = "your_SSID";
const char* password = "your_PASSWORD";

// Replace with your server URL
const char* serverUrl = "http://yourserver.com/api/post";

void setup() {
  Serial.begin(115200);

  // Connect to Wi-Fi
  WiFi.begin(ssid, password);
  Serial.print("Connecting to Wi-Fi");
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.print(".");
  }
  Serial.println();
  Serial.println("Connected to Wi-Fi");

  // Make the HTTP POST request
  if (WiFi.status() == WL_CONNECTED) {
    HTTPClient http;

    // Specify the URL
    http.begin(serverUrl);

    // Specify content-type header
    http.addHeader("Content-Type", "application/json");

    // Prepare JSON payload
    String payload = "{\"key1\":\"value1\",\"key2\":\"value2\"}";

    // Send POST request
    int httpResponseCode = http.POST(payload);

    // Check the response code
    if (httpResponseCode > 0) {
      String response = http.getString();
      Serial.print("HTTP Response code: ");
      Serial.println(httpResponseCode);
      Serial.print("Response: ");
      Serial.println(response);
    } else {
      Serial.print("Error code: ");
      Serial.println(httpResponseCode);
    }

    // Free resources
    http.end();
  } else {
    Serial.println("Wi-Fi not connected");
  }
}

void loop() {
  // Put your main code here, to run repeatedly
}
