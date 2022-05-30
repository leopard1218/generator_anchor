const fs = require('fs');
const idl = require('./target/idl/generator.json');

fs.writeFileSync('./app/src/idl.json', JSON.stringify(idl));