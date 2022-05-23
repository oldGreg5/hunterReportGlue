# Huner report glue

Render description of different items from file,
then user picks desired ones and they will be combined and put in clipboard

## Dev convenience:

### watchify to build browserify bundle on changes
watchify ./scripts/hunterReport.js -o ./scripts/bundle.js -v
### start node server
nodemon testNode.js
or
pm2 start testNode.js --watch

### make pm2 start with system
once app is running, check pm2 status
pm2 startup
pm2 save

### access app
port forwarding has to be done to vm hosting app
http://babilonas.myqnapcloud.com:8089/hunterReport