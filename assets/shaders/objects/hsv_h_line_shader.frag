#version 330 core

in VS_OUTPUT {
    float h;
} IN;

out vec4 FragColor;

void main()
{

  float r = 0.0;
  float g = 0.0;
  float b = 0.0;

  float h = IN.h;

  if (h <= 60.0) { // r fixed with g
    r = 1.0;
    g = h / 60.0;
  }
  else if (h <= 120.0) { // g fixed with r
    r = 1.0 - ((h - 60.0)/ 60.0);
    g = 1.0;
  }
  else if (h <= 180.0) { //  g fixed with b
    g = 1.0;
    b = (h - 120.0)/ 60.0;
  }
  else if (h <= 240.0) { // b fixed with g
    b = 1.0;
    g = 1.0 - (h - 180.0) / 60.0;

  }
  else if (h <= 300.0) { // b fixed with r
    b = 1.0;
    r = (h - 240.0)/60.0;
  }
  else { // r fixed with b
    r = 1.0;
    b = 1.0 - (h - 300.0)/ 60.0;
  }

  FragColor = vec4(r, g, b, 1.0);
}
