# https://github.com/Chlumsky/msdf-atlas-gen
# https://github.com/Blatko1/awesome-msdf


.\msdf-atlas-gen.exe -font %1.ttf  -size %3 -type %2 -imageout %2_%1.png -fontname %2_%1 -json  %2_%1.json

xcopy %2_%1.json ..\fonts\%2_%1_%3.json /Y
xcopy %2_%1.png ..\fonts\%2_%1_%3.png /Y