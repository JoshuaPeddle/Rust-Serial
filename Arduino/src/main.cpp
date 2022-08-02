#include <Arduino.h>

#define BAUD_RATE   57600

// Setup, initialize 
void setup() 
{
  Serial.begin(BAUD_RATE);  
  while (!Serial) {
    ; // wait for serial port to connect. Needed for Leonardo only
  }
}

// Loop forever
void loop() 
{
  // Serial data is pending
  while (Serial.available()) 
  { 
    // Echo serial data on serial device
    Serial.write(Serial.read());
    //int randomNumber = random(20, 1000);
    //delay(randomNumber);
    //Serial.write( "yeet" );
  }  
}