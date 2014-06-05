var rs = require('randomstring');

/* original hashCode - provided by mako */
var hashCode = function(str){
  var hash = 0, i, char;
  if (str.length == 0) return hash;
  for (var i = 0, l = str.length; i < l; i++) {
    char  = str.charCodeAt(i);
    hash  = ((hash<<5)-hash)+char;
    hash |= 0; // Convert to 32bit integer
  }
  return hash;
};

var str = "", hash = 0, done = 0;

//while (hash != 666) {
for (var i = 0; i < 100000; i++) {
  done += 1;
  if (done % 1000 == 0) {
    console.error('!gc');
    global.gc();
  }

  str = rs.generate(Math.ceil(Math.random() * 19));
  hash = hashCode(str);
  console.log(done + ": " + str + " = " + hash);
}
