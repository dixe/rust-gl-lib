# https://github.com/Chlumsky/msdf-atlas-gen
# https://github.com/Blatko1/awesome-msdf


.\msdf-atlas-gen.exe -font C:\Windows\Fonts\arial.ttf  -size 32 -type mtsdf -imageout msdf_arial.png -fontname arial -json  msdf_arial.json

xcopy msdf_arial.json ..\fonts\msdf_arial.json /Y
xcopy msdf_arial.png ..\fonts\msdf_arial.png /Y