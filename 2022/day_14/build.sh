tsc index.ts;
rm index.html;
touch index.html;
echo  '<head></head><body><canvas id="Main" width="1000" eight="1000px"><script>' >> index.html;
cat index.js >> index.html;
echo '</script></canvas></body>' >> index.html;